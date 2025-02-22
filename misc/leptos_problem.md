**Describe the bug**
Removing item from list rendered via <For/> component may result in error _Uncaught Error: closure invoked recursively or after being dropped_ if the removed item had an _on:_ event which triggered later.

**Leptos Dependencies**

```toml
leptos = { version = "0.7.4" }
leptos_router = { version = "0.7.4" }
leptos_axum = { version = "0.7.4", optional = true }
leptos_meta = { version = "0.7.4" }
wasm-bindgen = "0.2.100"
web-sys= { version = "0.3" }
```

**To Reproduce**

1. Drag any item (item_0, item_0) to div.bin (gray box on the right)
2. Get _Uncaught Error: closure invoked recursively or after being dropped_ in console
3. Get the item removed from Vec

**Expected behavior**
1. Drag any item (item_0, item_0) to div.bin (gray box on the right)
2. Get the item removed from Vec

**Additional context**
If there was't not on:dragend trigger, there wouldn't be an error with "steps to reproduce". I've come to the conclusion, that closure is triggered for already removed element.

**Minimal code sample**

```rust
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
                                        // Commenting this expression removes error
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

```

**Styling**
```css

.dragndrop>.wrapper {
    height: 100vh;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-around;
    background-color: black;
}

.dragndrop>.wrapper>.box {
    background-color: orange;
    width: 50%;
    height: 50vh;
}

.dragndrop>.wrapper>.box>.items {
    display: flex;
    flex-direction: column;
    width: 50%;
    overflow-y: auto;
    height: 100%;
}

.dragndrop>.wrapper>.box>.items>.item {
    word-wrap: break-word;
    background-color: orangered;
    width: 100%;
    padding: 1rem;
    border-bottom: 2px solid white;
    user-select: none;
    cursor: pointer;
}

.dragndrop>.wrapper>.box>.items>.item:active {
    background-color: purple;
}

.dragndrop>.wrapper>.bin {
    background-color: gray;
    width: 25%;
    height: 25vh;
}
```
