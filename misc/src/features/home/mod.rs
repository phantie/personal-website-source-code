#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};
pub mod components;

#[component(transparent)]
pub fn HomeRoutes() -> impl MatchNestedRoutes + Clone {
    use components::home::Home;
    use components::old_home::OldHome;

    view! {
        <Route path=StaticSegment("/") view=OldHome />
        <Route path=StaticSegment("/h") view=Home />
    }
    .into_inner()
}
