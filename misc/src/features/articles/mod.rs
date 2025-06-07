use leptos::prelude::*;
#[allow(unused)]
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes,
};
pub mod components;
pub mod defs;
pub mod fns;
pub mod instances;
pub mod server_fns;

#[component(transparent)]
pub fn ArticleRoutes() -> impl MatchNestedRoutes + Clone {
    use crate::features::articles::components::article::Article;
    use crate::features::articles::components::article_list::ArticleList;

    view! {
        <ParentRoute path=path!("/articles") view=ArticleList ssr=leptos_router::SsrMode::PartiallyBlocked>
            <Route path=path!(":id") view=Article ssr=leptos_router::SsrMode::PartiallyBlocked />
            <Route path=path!("") view=Article ssr=leptos_router::SsrMode::PartiallyBlocked />
        </ParentRoute>
    }
    .into_inner()
}
