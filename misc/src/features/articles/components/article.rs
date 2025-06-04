use leptos::logging::log;
use leptos::prelude::*;
use leptos::Params;
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
use crate::features::articles::instances::NOT_FOUND_ARTICLE_ID;
use crate::features::articles::server_fns::{get_any_article_id, get_article, get_article_content};

#[component]
pub fn Article() -> impl IntoView {
    let get_article_id = Resource::new_blocking(article_id(), |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        };

        id
    });

    // let get_article_content_resource = Resource::new_blocking(id_memo, |id| async move {
    let get_article_content_resource = Resource::new(article_id(), |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        }?;

        get_article_content(id).await
    });

    let get_article_resource = Resource::new_blocking(article_id(), |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        }?;

        get_article(id).await
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

                    let description_meta = view! { <Meta name="description" content={article.description.unwrap_or_default()}/> };
                    let keywords_meta = view! { <Meta name="keywords" content={article.tags.join(", ")} /> };

                    // log!("applying suspense");

                    // only apply meta tags on SSR
                    #[cfg(feature = "ssr")]
                    {
                        return view! {
                            <Title text={article.title} />
                            {description_meta}
                            {keywords_meta}
                        }.into_any()
                    }

                    #[cfg(not(feature = "ssr"))]
                    {
                        return view! {
                            <Title text={article.title} />
                        }.into_any()
                    }

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
    js::anchors::add_default();
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
