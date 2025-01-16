#![allow(unused)]

use leptos::prelude::*;
use leptos::Params;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};
mod defs;
use defs::*;

/// Renders the article list
#[component]
fn ArticleList() -> impl IntoView {
    view! {
        <h1>"Article list"</h1>
        <Outlet/>
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

    view! {
        <h2>"Some Article "{ id }</h2>

        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || get_article_content_resource.get().map(|a| view! { <p> {a} </p> })}
        </Suspense>

    }
}

#[component]
fn NoArticle() -> impl IntoView {
    view! {
        <h3>"No article"</h3>
    }
}

#[server]
pub async fn get_article_content(article_id: ArticleId) -> Result<ArticleContent, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    // TODO get from env
    let base_path: String = "/Users/phantie/Projects/misc/misc/src/features/articles/md".into();

    let relative_sources = RelativeLocalArticleSources::default();

    let local_source = LocalArticleSource {
        base_path,
        relative_source: relative_sources.get_by_id(&article_id).clone(),
    };

    let content = local_source.load_article_content();

    Ok(format!(
        "Loaded article ID {article_id}\nContent: {content}"
    ))
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
