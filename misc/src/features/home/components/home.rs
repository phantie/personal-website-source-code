#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
    <link href="https://fonts.googleapis.com/css2?family=Dancing+Script:wght@400..700&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet" />
    <div class="home-container">
        <div class="home-sections">
            <div class="home-sections-title"><h1>{"Sections"}</h1></div>

            <div class="home-sections-sections">
                <div class="home-sections-engineering">
                <h2>{"Engineering"}</h2>
                </div>

                <div class="home-sections-life">
                <h2>{"Life"}</h2>
                </div>
            </div>
        </div>

        <div class="home-links">
            <div class="home-links-title"><h1>{"Links"}</h1></div>

            <div class="home-links-sections">
                <div class="home-links-youtube">
                <h2>{"YouTube"}</h2>
                </div>

                <div class="home-links-github">
                <h2>{"GitHub"}</h2>
                </div>

                <div class="home-links-telegram">
                <h2>{"Telegram"}</h2>
                </div>
            </div>
        </div>
    </div>

    }
}
