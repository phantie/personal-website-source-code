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

fn parse_md(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
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
                        view! { <div class="markdown-body" inner_html={raw_html}></div> }
                    })
                }
            </Suspense>
        </div>
        <HighlightCode/>
    }
}

mod hljs {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll)]
        pub fn hljs_highlight_all();

        #[wasm_bindgen(js_namespace = anchors, js_name = add)]
        pub fn anchors_add();

        #[wasm_bindgen(js_namespace = anchors, js_name = add)]
        pub fn anchors_add_arg(value: String);
    }
}

/// Thanks to https://github.com/metatoaster/leptos_axum_js_ssr/
#[component]
fn HighlightCode() -> impl IntoView {
    use hljs::{anchors_add, anchors_add_arg, hljs_highlight_all};
    view! {
        <Suspense fallback=move || view! {}>{
            move || Suspend::new(async move {
                Effect::new(move |_| {
                    request_animation_frame(move || {
                        leptos::logging::log!("request_animation_frame invoking hljs and anchors");
                        // under SSR this is an noop, but it wouldn't be called under there anyway because
                        // it isn't the isomorphic version, i.e. Effect::new_isomorphic(...).
                        hljs_highlight_all();
                        anchors_add();
                        anchors_add_arg(".articles-article h1".into());
                    });
                });
                view! {}
            })
        }</Suspense>
    }
    .into_any()
}
