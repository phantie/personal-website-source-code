use crate::features::articles::components::params::article_id;
use crate::features::articles::defs::*;
use crate::features::articles::server_fns::get_preload_images_links;
use leptos::{logging::log, prelude::*};
use leptos_router::components::{Outlet, A};

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

    let get_preload_images_els = |links: Vec<String>| {
        links
            .into_iter()
            .map(|link| {
                view! {
                    <link rel="prefetch" href={link} r#as="image"/>
                }
            })
            .collect::<Vec<_>>()
    };

    let get_preload_images_links_resource =
        Resource::new(|| (), |id| async move { get_preload_images_links(5).await });

    view! {
        <div class="articles">
            <div class="articles-list" class:focus={move || article_id()().is_none() }>
            <h1>"Posts"</h1>
                <div class="articles-list-items">
                {article_list}
                </div>
            </div>
            <Outlet/>

            <Suspense fallback=move || view! {}>
                {move || get_preload_images_links_resource.get().map(|links|
                    {
                        links.map(|links| get_preload_images_els(links).into_any()).unwrap_or(view! {}.into_any())
                    })
                }
            </Suspense>
        </div>
    }
}
