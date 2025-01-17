use leptos::prelude::*;
#[allow(unused)]
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes,
};
mod components;
mod defs;
mod server_fns;

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
