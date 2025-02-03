#![allow(unused)]
use leptos::{logging::log, prelude::*};
use std::collections::HashMap;
use web_sys::DragEvent;

type Filename = String;

#[derive(Clone)]
struct DroppedFile {
    filename: Filename,
}

type Key = Filename;
type DroppedFiles = Vec<(Key, ArcRwSignal<DroppedFile>)>;

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
    let (dragged_over_rs, dragged_over_ws) = signal(false);
    let (last_dragged_rs, last_dragged_ws) = signal::<Option<Key>>(None);

    let m = vec![
        (
            "default_1".to_owned(),
            ArcRwSignal::new(DroppedFile {
                filename: "default_1".to_owned(),
            }),
        ),
        (
            "default_2".to_owned(),
            ArcRwSignal::new(DroppedFile {
                filename: "default_2".to_owned(),
            }),
        ),
    ];

    let (dropped_files_rs, dropped_files_ws) = signal::<DroppedFiles>(m);

    view! {
        <div class="dragndrop">
            <div class="wrapper">

                <div
                    class="box"
                    class:dragged_over=move || dragged_over_rs.get()
                    on:dragenter=move |e: DragEvent| {
                        dragged_over_ws.set(true);
                        // log!("dragenter");
                    }
                    on:dragleave=move |e: DragEvent| {
                        dragged_over_ws.set(false);
                        // log!("dragleave");
                    }
                    on:dragover=move |e: DragEvent| {
                        e.prevent_default();
                    }
                    on:drop=move |e: DragEvent| {
                        dragged_over_ws.set(false);
                        // log!("drop");
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
                                            fs.push((key, ArcRwSignal::new(DroppedFile {
                                                filename,
                                            })));
                                        });
                                    }
                                }
                            }
                        }
                    }
                >


                <div class="dropped_files">
                    <For
                        each=move || dropped_files_rs.get()
                        key=|(key, dropped_file)| key.clone()
                        children=move |(key, dropped_file)| {
                            let s = RwSignal::from(dropped_file);
                            view! {
                                <div
                                    class="dropped_file"
                                    draggable="true"
                                    on:drag=move |e| {
                                        // log!("drag");
                                    }
                                    on:dragstart=move |e| {
                                        log!("dragstart");
                                        last_dragged_ws.set(Some(key.clone()));
                                    }
                                >
                                    {move || s.get().filename}
                                </div>
                            }
                        }
                    />
                </div>


                </div>

                <div
                    class="bin"
                    on:dragenter=move |e: DragEvent| {
                        // log!("dragenter");
                    }
                    on:dragleave=move |e: DragEvent| {
                        // log!("dragleave");
                    }
                    on:dragover=move |e: DragEvent| {
                        e.prevent_default();
                    }
                    on:drop=move |e: DragEvent| {
                        if let Some(dragged) = last_dragged_rs.read_untracked().clone() {
                            log!("want to remove {dragged:?}");

                            dropped_files_ws.write().retain(|(k, _)| k != &dragged);
                        }
                    }
                >
                </div>
            </div>
        </div>
    }
}
