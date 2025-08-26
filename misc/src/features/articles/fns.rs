#![allow(unused)]
use crate::features::articles::components::params::article_id;
use crate::features::articles::defs::ArticleCategory;
use crate::features::articles::defs::*;
use crate::features::articles::server_fns::get_preload_images_links;
use leptos::{logging::log, prelude::*};
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_query_map;

pub fn get_article_category_from_query_params() -> Option<ArticleCategory> {
    let query = use_query_map();
    let value = query
        .with(|q| q.get("category"))
        .map(|v| ArticleCategory::try_from(v.as_str()).ok())
        .flatten();

    value
}
