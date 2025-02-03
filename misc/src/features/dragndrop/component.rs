#![allow(unused)]
use leptos::{logging::log, prelude::*};
use std::collections::HashMap;
use web_sys::DragEvent;

#[derive(Clone)]
struct DroppedFile {
    s: ArcRwSignal<String>,
    filename: String,
}

type DroppedFiles<K> = HashMap<K, DroppedFile>;

#[derive(Clone)]
struct BoxState {
    dragged_over: bool,
}

impl Default for BoxState {
    fn default() -> Self {
        Self {
            dragged_over: false,
        }
    }
}

#[component]
pub fn Component() -> impl IntoView {
    let (box_state_rs, box_state_ws) = signal(BoxState::default());

    let (dropped_files_rs, dropped_files_ws) = signal::<DroppedFiles<String>>(Default::default());

    view! {
        <div class="dragndrop">
            <div class="wrapper">

                <div
                    class="box"
                    class:dragged_over=move || box_state_rs.get().dragged_over
                    on:dragenter=move |e: DragEvent| {
                        box_state_ws.update(|s|
                            s.dragged_over = true
                        );

                        log!("dragenter");
                    }
                    on:dragleave=move |e: DragEvent| {
                        box_state_ws.update(|s|
                            s.dragged_over = false
                        );

                        log!("dragleave");
                    }
                    on:dragover=move |e: DragEvent| {
                        e.prevent_default();
                    }
                    on:drop=move |e: DragEvent| {
                        box_state_ws.update(|s|
                            s.dragged_over = false
                        );

                        log!("drop");
                        e.prevent_default();
                        if let Some(data_transfer) = e.data_transfer() {
                            if let Some(files) = data_transfer.files() {
                                for index in (0..files.length()) {
                                    let file = files.item(index);
                                    if let Some(file) = file {
                                        let filename = file.name();
                                        log!("dropped {:?}", filename);

                                        let key = filename.clone();

                                        dropped_files_ws.update(|fs| {
                                            fs.insert(key, DroppedFile {
                                                s: ArcRwSignal::new(filename.clone()),
                                                filename: filename
                                            });
                                        });
                                    }
                                }
                            }
                        }
                    }
                >


                <For
                    each=move || dropped_files_rs.get()
                    key=|(key, dropped_file)| key.clone()
                    children=move |(key, dropped_file)| {
                        let s = RwSignal::from(dropped_file.s);
                        view! {
                            <h1>{dropped_file.filename}</h1>
                        }
                    }
                />


                </div>
            </div>
        </div>
    }
}
