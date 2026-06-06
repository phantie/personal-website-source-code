fn site_url() -> String {
    std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".into())
}

#[cfg(feature = "ssr")]
async fn robots_txt_handler() -> impl axum::response::IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
        format!(
            "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
            site_url()
        ),
    )
}

#[cfg(feature = "ssr")]
async fn sitemap_xml_handler() -> impl axum::response::IntoResponse {
    use misc::features::articles::instances::get_articles_chronological_order;

    let base = site_url();
    let articles = get_articles_chronological_order();

    let urls: String = articles
        .iter()
        .map(|a| {
            format!(
                "  <url>\n    <loc>{base}/articles/{id}</loc>\n  </url>\n",
                id = a.id
            )
        })
        .chain(std::iter::once(format!(
            "  <url>\n    <loc>{base}/</loc>\n  </url>\n"
        )))
        .collect();

    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}</urlset>"
    );

    (
        [(
            axum::http::header::CONTENT_TYPE,
            "application/xml; charset=utf-8",
        )],
        xml,
    )
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use misc::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/robots.txt", axum::routing::get(robots_txt_handler))
        .route("/sitemap.xml", axum::routing::get(sitemap_xml_handler))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::from_fn(caching))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

async fn caching(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // get body preferably as hash
    // get body hash
    // form an Etag from the hash
    // compare incoming Etag with current Etag

    // let request_etag_header = request.headers().get(http::header::ETAG).cloned();
    let request_if_none_match_header = request.headers().get(http::header::IF_NONE_MATCH).cloned();

    #[allow(unused)]
    let uri_path = request.uri().path().to_string();

    // log!("{}", format!("{:?}", uri_path));

    let mut response = next.run(request).await;

    // interesting discovery std::mem::take
    let body = std::mem::take(response.body_mut());

    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap(); // TODO handle maybe?

    let body_hash = {
        use std::hash::Hasher;
        let mut hasher = ahash::AHasher::default();
        hasher.write(&body_bytes);
        let hash_value = hasher.finish();
        hash_value
    };

    let response_etag = body_hash.to_string();

    let matches_etag = match request_if_none_match_header {
        Some(value) => match value.to_str() {
            Ok(value) => response_etag.as_str() == value,
            Err(_) => false,
        },
        None => false,
    };

    // after std::mem::take you must place a value from where it was taken
    *response.body_mut() = axum::body::Body::from(body_bytes);

    async fn handle_caching(
        mut response: axum::response::Response,
        matches_etag: bool,
        response_etag: &str,
    ) -> axum::response::Response {
        if matches_etag {
            use axum::response::IntoResponse;

            (
                http::StatusCode::NOT_MODIFIED,
                [
                    (http::header::ETAG, response_etag),
                    (http::header::CACHE_CONTROL, "no-cache, must-revalidate"),
                ],
                axum::body::Body::empty(),
            )
                .into_response()
        } else {
            response
                .headers_mut()
                .insert(http::header::ETAG, response_etag.try_into().unwrap());

            response.headers_mut().insert(
                http::header::CACHE_CONTROL,
                "no-cache, must-revalidate".try_into().unwrap(),
            );

            response
        }
    }

    // log!(
    //     "{}",
    //     format!(
    //         "uri_path {:?}, response content_type {:?}, matches_etag {:?} ",
    //         uri_path,
    //         response.headers().get(axum::http::header::CONTENT_TYPE),
    //         matches_etag,
    //     )
    // );

    // Post-processing of the response:
    if let Some(content_type) = response.headers().get(axum::http::header::CONTENT_TYPE) {
        if let Ok(content_type_str) = content_type.to_str() {
            let is_wasm = content_type_str.contains("application/wasm");
            let is_js = content_type_str.contains("text/javascript");
            let is_css = content_type_str.contains("text/css");
            let is_image = content_type_str.contains("image/");

            if is_wasm || is_js || is_css || is_image {
                let response = handle_caching(response, matches_etag, response_etag.as_str()).await;
                return response;
            }
        }
    }

    response
}
