#![allow(unused)]
use crate::features::articles::components::params::article_id;
use crate::features::articles::defs::ArticleCategory;
use crate::features::articles::defs::*;
use crate::features::articles::server_fns::get_preload_images_links;
use leptos::{logging::log, prelude::*};
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_query_map;

pub fn get_article_category() -> Option<ArticleCategory> {
    let query = use_query_map();
    match query.with(|q| q.get("category")).as_deref() {
        Some("engineering") => Some(ArticleCategory::Engineering),
        Some("life") => Some(ArticleCategory::Life),
        _ => None,
    }
}
