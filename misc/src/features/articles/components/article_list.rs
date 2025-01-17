#![allow(unused)]

use crate::features::articles::defs::*;
use crate::features::articles::server_fns::{get_any_article_id, get_article, get_article_content};
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

/// Renders the article list
#[component]
pub fn ArticleList() -> impl IntoView {
    let articles = Articles::default();

    let article_list = articles
        .ordered_articles
        .into_iter()
        // .cycle()
        // .take(20)
        .map(|article| {
            let url = format!("/articles/{}", article.id);
            view! {
                <div class="articles-list-item">
                    <A href=url>
                        <div class="articles-list-item-link">
                            {article.title}
                        </div>
                    </A>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="articles">
            <div class="articles-list">
            <h1>"Posts"</h1>
                <div class="articles-list-items">
                {article_list}
                </div>
            </div>
            <Outlet/>
        </div>
    }
}
