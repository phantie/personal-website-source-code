use crate::features::articles::defs::*;
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

#[server]
pub async fn get_article(article_id: ArticleId) -> Result<Article, ServerFnError> {
    let articles = Articles::default();
    let article = articles.get_by_id(article_id).clone();
    Ok(article)
}

#[server]
pub async fn get_any_article_id() -> Result<ArticleId, ServerFnError> {
    let articles = Articles::default();
    let article = articles.get_by_id("first".into());
    Ok(article.id.clone())
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

fn get_base_path() -> &'static str {
    // TODO get from env
    "/Users/phantie/Projects/misc/misc/src/features/articles/md"
}
