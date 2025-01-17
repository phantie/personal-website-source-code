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
mod components;
mod defs;
mod server_fns;
use defs::*;
use server_fns::{get_any_article_id, get_article, get_article_content};

#[component(transparent)]
pub fn ArticleRoutes() -> impl MatchNestedRoutes + Clone {
    use crate::features::articles::components::article::Article;
    use crate::features::articles::components::article_list::ArticleList;

    view! {
        <ParentRoute path=path!("/articles") view=ArticleList>
            <Route path=path!(":id") view=Article />
            <Route path=path!("") view=Article />
        </ParentRoute>
    }
    .into_inner()
}
