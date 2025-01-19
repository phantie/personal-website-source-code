#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text={"phantie blog"} />
        <Redirect path="/articles" />
    }
}

#[component(transparent)]
pub fn HomeRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        // <Route path=StaticSegment("/") view=HomePage/>
        <Route path=StaticSegment("/") view=Home />
    }
    .into_inner()
}
