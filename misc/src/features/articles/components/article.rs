use leptos::html::article;
use leptos::prelude::*;
use leptos::Params;
use leptos_meta::Stylesheet;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes, StaticSegment,
};

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

    let get_article_resource = Resource::new(id_memo, |id| async move {
        let id = if let Some(id) = id {
            Ok(id)
        } else {
            let id = get_any_article_id().await;
            id
        }?;

        get_article(id).await
    });

    view! {
        // <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/3.0.1/github-markdown.min.css"/>
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.8.1/github-markdown.min.css"/>
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

    }
}
