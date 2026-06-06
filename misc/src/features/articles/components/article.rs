use leptos::logging::log;
use leptos::prelude::*;
use leptos::Params;
use leptos_meta::Link;
use leptos_meta::Meta;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
use wasm_bindgen::JsCast;
use web_sys::Event;

use crate::features::articles::components::params::article_id;
use crate::features::articles::defs::*;
use crate::features::articles::fns::get_article_category_from_query_params;
use crate::features::articles::instances::NOT_FOUND_ARTICLE_ID;
use crate::features::articles::server_fns::{
    get_article, get_article_content, get_latest_article_id,
};
use leptos_router::hooks::{use_location, use_navigate};

#[component]
pub fn Article() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();

    let article_category = get_article_category_from_query_params();

    let get_article_id = Resource::new_blocking(article_id(), move |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_latest_article_id(article_category).await;
            id
        };

        id
    });

    let get_article_content_resource = Resource::new_blocking(article_id(), move |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_latest_article_id(article_category).await;
            id
        }?;

        get_article_content(id).await
    });

    let get_article_resource = Resource::new_blocking(article_id(), move |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_latest_article_id(article_category).await;
            id
        }?;

        get_article(id).await
    });

    // Effect to automatically add category query param when article loads
    Effect::new({
        let navigate = navigate.clone();
        let location = location.clone();
        move |_| {
            if let Some(Ok(article)) = get_article_resource.get() {
                let current_pathname = location.pathname.get();
                let current_search = location.search.get();

                // Only preserve category if we're in the /articles/* namespace
                if current_pathname.starts_with("/articles/") {
                    // Check if category param already exists
                    if !current_search.contains("category=") {
                        let category = article.category.to_string();

                        let new_url = if current_search.is_empty() {
                            format!("{}?category={}", current_pathname, category)
                        } else {
                            format!(
                                "{}{}&category={}",
                                current_pathname, current_search, category
                            )
                        };

                        navigate(&new_url, Default::default());
                    }
                }
            }
        }
    });

    let (article_content_loaded_rs, article_content_loaded_ws) = signal(false);
    let (scroll_progress_rs, scroll_progress_ws) = signal(0.0);

    let handle_content_scroll = move |e: Event| {
        let target = e.target().unwrap();
        let element = target.dyn_ref::<web_sys::HtmlElement>().unwrap();

        let scroll_top = element.scroll_top() as f64;
        let scroll_height = element.scroll_height() as f64 - element.client_height() as f64;
        let progress = (scroll_top / scroll_height) * 100.0;
        scroll_progress_ws.set(progress);
    };

    view! {
        <Suspense fallback=move || ()>
        {
            move || Suspend::new(async move {
                let article = get_article_resource.await;

                if let Ok(article) = article {

                    #[cfg(feature = "ssr")]
                    {
                        if article.id == NOT_FOUND_ARTICLE_ID {
                            use leptos_axum::ResponseOptions;
                            use http::StatusCode;
                            let response = expect_context::<ResponseOptions>();
                            response.set_status(StatusCode::NOT_FOUND);
                        }
                    }

                    let (description_for_ld, tags_for_ld, created_at_ld, written_on_ld) = (
                        article.description.clone().unwrap_or_default(),
                        article.tags.clone(),
                        article.created_at,
                        article.written_on,
                    );

                    let meta_tags = view! {
                        <Meta name="description" content={description_for_ld.clone()}/>
                        <Title text={article.title.clone()}/>
                        <Meta property="og:type" content="website"/>
                        <Meta property="og:title" content={article.title.clone()}/>
                        <Meta property="og:description" content={description_for_ld.clone()}/>
                        <Meta name="twitter:card" content="summary"/>
                        <Meta name="twitter:title" content={article.title.clone()}/>
                        <Meta name="twitter:description" content={description_for_ld.clone()}/>
                    };

                    let keywords_meta = view! { <Meta name="keywords" content={tags_for_ld.join(", ")} /> };

                    let site_url = crate::site_url();
                    let article_url = format!("{}/articles/{}", site_url, article.id);

                    let json_ld = build_article_json_ld(
                        &article.title,
                        &description_for_ld,
                        &article_url,
                        &tags_for_ld,
                        created_at_ld,
                        written_on_ld,
                    );

                    view! {
                        <Title text={article.title} />
                        {meta_tags}
                        {keywords_meta}
                        <Meta property="og:url" content={article_url.clone()}/>
                        <Link rel="canonical" href={article_url}/>
                        <script type="application/ld+json" inner_html={json_ld}></script>
                    }.into_any()

                } else {
                    ().into_any()
                }
            })
        }
        </Suspense>

        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.8.1/github-markdown.min.css"/>

        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/default.min.css"/>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>

        <script src="https://cdn.jsdelivr.net/npm/anchor-js/anchor.min.js"></script>

        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-latte.css"/>
        <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-frappe.css"/>
        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-macchiato.css"/>
        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-mocha.css"/>

        // {
        //    move || {  if (id_memo().is_some()) {} else {} }
        // }


        <div
        class="articles-article"
        class:focus={move || article_id()().is_some() && article_content_loaded_rs.get() }
        >
            <div
                class="article-reading-progress"
                style:width=move || format!("{}%", scroll_progress_rs.get())
            ></div>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || get_article_content_resource.get().map(|content|
                    {
                        let raw_html = parse_md(&content.unwrap_or_else(|_| "<h2>Failed to load article content</h2>".to_owned()));

                        article_content_loaded_ws.set(true);
                        scroll_progress_ws.set(0.0);

                        view! {
                            <div
                                class="markdown-body"
                                inner_html={raw_html}
                                on:scroll=handle_content_scroll
                            ></div>
                            <ApplyJs/>
                        }
                    })
                }
            </Suspense>
        </div>
    }
}

mod js {

    pub fn scroll_to_hash_element() {
        use web_sys::{window, Document, ScrollBehavior, ScrollIntoViewOptions};

        // Get the current URL hash (the part after #)
        let location = window().unwrap().location();
        let hash = location.hash();

        // Check if there's a hash (non-empty)
        if let Ok(hash) = hash {
            if hash.is_empty() {
                return;
            }

            // Remove the "#" from the hash
            let id = &hash[1..];

            // Get the document
            let document = window().unwrap().document().unwrap();

            // Try to find the element with the ID
            if let Some(element) = document.get_element_by_id(id) {
                // Scroll the element into view
                let mut options = ScrollIntoViewOptions::new();
                options.set_behavior(ScrollBehavior::Instant);
                element.scroll_into_view_with_scroll_into_view_options(&options);
            }
        }
    }

    pub mod hljs {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace=hljs, js_name=highlightAll)]
            pub fn highlight_all();
        }
    }

    pub mod anchors {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace=anchors, js_name=add)]
            pub fn add_default();

            #[wasm_bindgen(js_namespace=anchors, js_name=add)]
            pub fn add_with_selector(selector: String);
        }
    }
}

fn apply_js() {
    js::hljs::highlight_all();
    js::anchors::add_with_selector("div.articles-article h2, div.articles-article h3, div.articles-article h4, div.articles-article h5, div.articles-article h6".to_string());

    js::scroll_to_hash_element();
}

/// Thanks to https://github.com/metatoaster/leptos_axum_js_ssr/
#[component]
fn ApplyJs() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! {}>{
            move || Suspend::new(async move {
                Effect::new(move |_| { request_animation_frame(apply_js); });
                view! {}
            })
        }</Suspense>
    }
    .into_any()
}

fn parse_md(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub(crate) fn build_article_json_ld(
    title: &str,
    description: &str,
    url: &str,
    tags: &[String],
    created_at: Option<chrono::NaiveDate>,
    written_on: Option<chrono::NaiveDate>,
) -> String {
    fn esc(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    let keywords_arr = tags
        .iter()
        .map(|t| format!("\"{}\"", esc(t)))
        .collect::<Vec<_>>()
        .join(",");

    let mut fields = vec![
        r#""@context":"https://schema.org""#.to_owned(),
        r#""@type":"BlogPosting""#.to_owned(),
        format!(r#""headline":"{}""#, esc(title)),
        format!(r#""description":"{}""#, esc(description)),
        r#""author":{"@type":"Person","name":"Alexander Tokar","url":"https://github.com/phantie"}"#.to_owned(),
        format!(r#""url":"{url}""#),
    ];

    if !keywords_arr.is_empty() {
        fields.push(format!(r#""keywords":[{keywords_arr}]"#));
    }
    if let Some(date) = created_at {
        fields.push(format!(r#""datePublished":"{}""#, date.format("%Y-%m-%d")));
    }
    if let Some(date) = written_on {
        fields.push(format!(r#""dateCreated":"{}""#, date.format("%Y-%m-%d")));
    }

    format!("{{{}}}", fields.join(","))
}

#[cfg(test)]
mod tests {
    use super::build_article_json_ld;
    use chrono::NaiveDate;

    fn date(y: i32, m: u32, d: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(y, m, d)
    }

    #[test]
    fn required_fields_present() {
        let json = build_article_json_ld("Title", "Desc", "https://phantie.dev/articles/foo", &[], None, None);
        assert!(json.starts_with('{') && json.ends_with('}'));
        assert!(json.contains(r#""@context":"https://schema.org""#));
        assert!(json.contains(r#""@type":"BlogPosting""#));
        assert!(json.contains(r#""headline":"Title""#));
        assert!(json.contains(r#""description":"Desc""#));
        assert!(json.contains(r#""url":"https://phantie.dev/articles/foo""#));
        assert!(json.contains(r#""author""#));
    }

    #[test]
    fn special_chars_escaped() {
        let json = build_article_json_ld(r#"He said "hello""#, "line1\nline2", "https://e.com", &[], None, None);
        assert!(json.contains(r#""headline":"He said \"hello\"""#));
        assert!(json.contains(r#""description":"line1\nline2""#));
    }

    #[test]
    fn dates_formatted_iso() {
        let json = build_article_json_ld("T", "", "https://e.com", &[], date(2025, 6, 5), date(2023, 8, 7));
        assert!(json.contains(r#""datePublished":"2025-06-05""#));
        assert!(json.contains(r#""dateCreated":"2023-08-07""#));
    }

    #[test]
    fn no_date_fields_when_none() {
        let json = build_article_json_ld("T", "", "https://e.com", &[], None, None);
        assert!(!json.contains("datePublished"));
        assert!(!json.contains("dateCreated"));
    }

    #[test]
    fn tags_serialized_as_array() {
        let json = build_article_json_ld("T", "", "https://e.com", &["rust".into(), "leptos".into()], None, None);
        assert!(json.contains(r#""keywords":["rust","leptos"]"#));
    }

    #[test]
    fn empty_tags_no_keywords_field() {
        let json = build_article_json_ld("T", "", "https://e.com", &[], None, None);
        assert!(!json.contains("keywords"));
    }
}
