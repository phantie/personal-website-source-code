use leptos::logging::log;
use leptos::prelude::*;
use leptos::Params;
use leptos_meta::Stylesheet;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;

use crate::features::articles::defs::*;
use crate::features::articles::server_fns::{get_any_article_id, get_article, get_article_content};

#[derive(Params, PartialEq)]
struct ArticleParams {
    id: Option<ArticleId>,
}

#[component]
pub fn Article() -> impl IntoView {
    let params = use_params::<ArticleParams>();

    let id_memo = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
    };

    // let get_article_content_resource = Resource::new_blocking(id_memo, |id| async move {
    let get_article_content_resource = Resource::new(id_memo, |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        }?;

        get_article_content(id).await
    });

    let _get_article_resource = Resource::new(id_memo, |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        }?;

        get_article(id).await
    });

    view! {
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.8.1/github-markdown.min.css"/>

        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/default.min.css"/>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"></script>

        <script src="https://cdn.jsdelivr.net/npm/anchor-js/anchor.min.js"></script>

        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-latte.css"/>
        <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-frappe.css"/>
        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-macchiato.css"/>
        // <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/@catppuccin/highlightjs@1.0.0/css/catppuccin-mocha.css"/>

        <div class="articles-article">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || get_article_content_resource.get().map(|content|
                    {
                        let raw_html = parse_md(&content.unwrap()); // TODO handle
                        view! {
                            <div class="markdown-body" inner_html={raw_html}></div>
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
            #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll)]
            pub fn highlight_all();
        }
    }

    pub mod anchors {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = anchors, js_name = add)]
            pub fn add();

            #[wasm_bindgen(js_namespace = anchors, js_name = add)]
            pub fn add_arg(value: String);
        }
    }
}

fn apply_js() {
    js::hljs::highlight_all();
    js::anchors::add();
    js::anchors::add_arg(".articles-article h1".into());
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
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
