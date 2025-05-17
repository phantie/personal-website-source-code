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
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(axum::middleware::from_fn(cache_control))
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

async fn cache_control(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;

    // Post-processing of the response:
    if let Some(content_type) = response.headers().get(axum::http::header::CONTENT_TYPE) {
        if let Ok(ct_str) = content_type.to_str() {
            let is_wasm = ct_str.contains("application/wasm");
            let is_js = ct_str.contains("javascript");
            let is_css = ct_str.contains("text/css");
            let is_image = ct_str.contains("image/");

            if is_wasm || is_js || is_css {
                // the easiest way
                response.headers_mut().insert(
                    axum::http::header::CACHE_CONTROL,
                    "no-store, no-cache".parse().unwrap(),
                );
            }

            if is_image {
                let max_age_in_seconds = {
                    let seconds_in_a_minute = 60;
                    let minutes_in_an_hour = 60;
                    let hours_in_a_day = 24;
                    let days = 2;

                    // cache for 2 days
                    std::time::Duration::from_secs(
                        seconds_in_a_minute * minutes_in_an_hour * hours_in_a_day * days,
                    )
                    .as_secs()
                };

                response.headers_mut().insert(
                    axum::http::header::CACHE_CONTROL,
                    format!("public, max-age={max_age_in_seconds}")
                        .parse()
                        .unwrap(),
                );
            }
        }
    }

    response
}
