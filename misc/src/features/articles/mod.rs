#![allow(unused)]

use leptos::html::article;
use leptos::prelude::*;
use leptos::Params;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes, StaticSegment,
};
mod defs;
use defs::*;

/// Renders the article list
#[component]
fn ArticleList() -> impl IntoView {
    let article_list = vec!["first", "not_found"]
        .into_iter()
        .map(|article_id| {
            let url = format!("/articles/{article_id}");
            view! {
                <div class="articles-list-item">
                    <A href=url>{article_id}</A>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="articles">
            <div class="articles-list">
                <h1>"Article list"</h1>
                {article_list}
            </div>
            <Outlet/>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct ArticleParams {
    id: Option<ArticleId>,
}

#[component]
fn Article() -> impl IntoView {
    let params = use_params::<ArticleParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };

    let get_article_content_resource =
        Resource::new(id, |id| async move { get_article_content(id).await });

    let get_article_resource = Resource::new(id, |id| async move { get_article(id).await });

    view! {
        <div class="articles-article">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || get_article_resource.get().map(|article|
                    match article {
                        Ok(article) => view! { <h1> {article.title} </h1> }.into_any(),
                        Err(_) => view! {}.into_any()
                    }
                )}
            </Suspense>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || get_article_content_resource.get().map(|content| view! { <p> {content} </p> })}
            </Suspense>
        </div>

    }
}

#[component]
fn NoArticle() -> impl IntoView {
    view! {}
}

#[server]
pub async fn get_article(article_id: ArticleId) -> Result<Article, ServerFnError> {
    let articles = Articles::default();
    let article = articles.get_by_id(article_id).clone();
    Ok(article)
}

#[server]
pub async fn get_article_content(article_id: ArticleId) -> Result<ArticleContent, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let articles = Articles::default();

    let article = articles.get_by_id(article_id);

    let relative_source = article.relative_source.clone();

    let local_source = LocalArticleSource {
        base_path: get_base_path().into(),
        relative_source,
    };

    let content = local_source.load_article_content();

    Ok(content)
}

#[component(transparent)]
pub fn ArticleRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/articles") view=ArticleList>
            <Route path=path!(":id") view=Article/>
            <Route path=path!("") view=NoArticle/>
        </ParentRoute>
    }
    .into_inner()
}

fn get_base_path() -> &'static str {
    // TODO get from env
    "/Users/phantie/Projects/misc/misc/src/features/articles/md"
}
