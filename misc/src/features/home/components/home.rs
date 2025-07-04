#![allow(unused)]

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Redirect, Route, Router, Routes},
    path, MatchNestedRoutes, StaticSegment,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Alexander Tokar's Blog" />

        <Meta name="description" content="Blog about life and software engineering" />
        <Meta
            name="keywords"
            content="software engineering, functional programming, life, poetry, thoughts"
        />

        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            href="https://fonts.googleapis.com/css2?family=Dancing+Script:wght@400..700&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap"
            rel="stylesheet"
        />

        <div class="home-container">
            <div class="home-top-section">
                <div class="home-sections">
                    <div class="home-sections-title">
                        <h1>{"Sections"}</h1>
                    </div>

                    <div class="home-sections-sections">
                        <a href="/articles?category=engineering" class="home-sections-engineering">
                            <h2>{"Engineering"}</h2>
                        </a>

                        <a href="/articles?category=life" class="home-sections-life">
                            <h2>{"Life"}</h2>
                        </a>
                    </div>
                </div>

                <div class="home-links">
                    <div class="home-links-title">
                        <h1>{"Links"}</h1>
                    </div>

                    <div class="home-links-sections">
                        <a
                            href="https://github.com/phantie"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="home-links-github"
                        >
                            <h2>{"GitHub"}</h2>
                        </a>

                        <a
                            href="https://www.youtube.com/@government_authority"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="home-links-youtube"
                        >
                            <h2>{"YouTube"}</h2>
                        </a>

                        <a
                            href="/articles/photography"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="home-links-instagram"
                        >
                            <h2>{"Instagram"}</h2>
                        </a>

                        <a
                            href="https://t.me/phantie"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="home-links-telegram"
                        >
                            <h2>{"Telegram"}</h2>
                        </a>
                    </div>
                </div>
            </div>

            <div class="home-image-section">
                <img src="/static/home/a_resolver.jpg" alt="A Resolver" class="home-image" />
            </div>
        </div>
    }
}
