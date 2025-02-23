use leptos::logging::log;
use leptos::prelude::*;
use leptos::Params;
use leptos_meta::Meta;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;

use crate::features::articles::defs::*;
use crate::features::articles::server_fns::{get_any_article_id, get_article, get_article_content};

#[derive(Params, PartialEq)]
struct ArticleParams {
    id: Option<ArticleId>,
}

pub fn article_id() -> impl Fn() -> Option<String> {
    let params = use_params::<ArticleParams>();

    let id_memo = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
    };

    id_memo
}
