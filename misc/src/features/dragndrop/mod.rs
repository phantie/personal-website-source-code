pub mod component;

use leptos::prelude::*;
use leptos_router::components::Outlet;
#[allow(unused)]
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes, A},
    path, MatchNestedRoutes,
};

#[component(transparent)]
pub fn DrangdropRoutes() -> impl MatchNestedRoutes + Clone {
    use component::Component;
    view! {
        <ParentRoute path=path!("/experiment/dragndrop") view=Outlet>
            <Route path=path!("") view=Component />
        </ParentRoute>
    }
    .into_inner()
}
