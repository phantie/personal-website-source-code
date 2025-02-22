/// using pattern https://book.leptos.dev/view/04_iteration.html#dynamic-rendering-with-the-for-component
use leptos::{logging::log, prelude::*};
use web_sys::DragEvent;

type Key = String;

type Items = Vec<(Key, ())>;

#[component]
pub fn Component() -> impl IntoView {
    let m = vec![("item_0".to_owned(), ()), ("item_1".to_owned(), ())];

    // List of items
    let (items_rs, items_ws) = signal::<Items>(m);

    // Last dragged item
    let (dragged_rs, dragged_ws) = signal::<Option<Key>>(None);

    view! {
        <div class="dragndrop">
            <div class="wrapper">

                <div class="box">
                    <div class="items">
                        <For
                            each=move || items_rs.get()
                            key=|(key, _)| key.clone()
                            children=move |(key, _)| {
                                view! {
                                    <div
                                        class="item"
                                        draggable="true"
                                        on:dragstart=move |_| {
                                            log!("dragstart");
                                            dragged_ws.set(Some(key.clone()));
                                        }
                                        // Commenting expression removes error
                                        on:dragend=move |_| {
                                            log!("dragend");
                                            dragged_ws.set(None);
                                        }
                                    >
                                        {key.clone()}
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                <div
                    class="bin"
                    on:dragover=move |e: DragEvent| {
                        e.prevent_default();
                    }
                    on:drop=move |_: DragEvent| {
                        if let Some(key) = dragged_rs.read_untracked().clone() {
                            log!("want to remove {key:?}");
                            items_ws.write().retain(|(k, _)| k != &key);
                        }
                    }
                >
                </div>
            </div>
        </div>
    }
}
