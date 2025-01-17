#![allow(unused)]

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
mod defs;
mod server_fns;
use defs::*;
use server_fns::{get_any_article_id, get_article, get_article_content};

/// Renders the article list
#[component]
fn ArticleList() -> impl IntoView {
    let articles = Articles::default();

    let article_list = articles
        .ordered_articles
        .into_iter()
        // .cycle()
        // .take(20)
        .map(|article| {
            let url = format!("/articles/{}", article.id);
            view! {
                <div class="articles-list-item">
                    <A href=url>
                        <div class="articles-list-item-link">
                            {article.title}
                        </div>
                    </A>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="articles">
            <div class="articles-list">
            <h1>"Posts"</h1>
                <div class="articles-list-items">
                {article_list}
                </div>
            </div>
            <Outlet/>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct ArticleParams {
    id: Option<ArticleId>,
}

pub fn parse_md(markdown_input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[component]
fn Article() -> impl IntoView {
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

#[component(transparent)]
pub fn ArticleRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/articles") view=ArticleList>
            <Route path=path!(":id") view=Article />
            <Route path=path!("") view=Article />
        </ParentRoute>
    }
    .into_inner()
}
