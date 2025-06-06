#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};

#[component]
pub fn OldHome() -> impl IntoView {
    view! {
        <Title text={"phantie blog"} />
        <Redirect path="/articles" />
    }
}
