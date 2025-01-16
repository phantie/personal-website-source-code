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

/// Renders the article list
#[component]
fn ArticleList() -> impl IntoView {
    view! {
        <h1>"Article list"</h1>
        <Outlet/>
    }
}

type ArticleId = String;

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

    view! {
        <h2>"Some Article " { id }</h2>
    }
}

#[component]
fn NoArticle() -> impl IntoView {
    view! {
        <h3>"No article"</h3>
    }
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
