#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes, A},
    path, MatchNestedRoutes, StaticSegment,
};

#[component]
pub fn PrimaryHeader() -> impl IntoView {
    view! {
        <header class="primary">
            <a href="/h" class="home-link">
                <h2>"Home"</h2>
            </a>
        </header>
    }
}
