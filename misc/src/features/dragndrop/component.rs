#![allow(unused)]
use leptos::{logging::log, prelude::*};
use std::collections::HashMap;
use web_sys::DragEvent;

type Filename = String;

#[derive(Clone)]
struct DroppedFile {
    filename: Filename,
    dragged: bool,
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
    let (box_state_rs, box_state_ws) = signal(BoxState::default());

    // let (bin_dragover_rs, bin_dragover_ws) = signal(false);
    let (dragged_rs, dragged_ws) = signal::<Option<Key>>(None);
    let (remove_file_rs, remove_file_ws) = signal::<Option<Key>>(None);

    // let (dropped_files_rs, dropped_files_ws) = signal::<DroppedFiles<Key>>(Default::default());

    let m = vec![
        (
            "default_1".to_owned(),
            ArcRwSignal::new(DroppedFile {
                filename: "default_1".to_owned(),
                dragged: false,
            }),
        ),
        (
            "default_2".to_owned(),
            ArcRwSignal::new(DroppedFile {
                filename: "default_2".to_owned(),
                dragged: false,
            }),
        ),
    ];

    let (dropped_files_rs, dropped_files_ws) = signal::<DroppedFiles>(m);

    Effect::new(move |_| {
        if let Some(remove_file) = remove_file_rs.get() {
            dropped_files_ws.write().retain(|(k, _)| k != &remove_file);
        }
    });

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
                        // log!("dragenter");
                    }
                    on:dragleave=move |e: DragEvent| {
                        box_state_ws.update(|s|
                            s.dragged_over = false
                        );
                        // log!("dragleave");
                    }
                    on:dragover=move |e: DragEvent| {
                        e.prevent_default();
                    }
                    on:drop=move |e: DragEvent| {
                        box_state_ws.update(|s|
                            s.dragged_over = false
                        );
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
                                                 dragged: false
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
                                    class:dragged=move || s.get().dragged
                                    draggable="true"
                                    on:drag=move |e| {
                                        // log!("drag");
                                    }
                                    on:dragstart=move |e| {
                                        log!("dragstart");
                                        s.update(|dropped_file| dropped_file.dragged = true);
                                        dragged_ws.set(Some(key.clone()));
                                    }
                                    on:dragend=move |e| {
                                        log!("dragend");
                                        s.update(|dropped_file| dropped_file.dragged = false);
                                        dragged_ws.set(None);
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
                        if let Some(dragged) = dragged_rs.read_untracked().clone() {
                            log!("want to remove {dragged:?}");

                            // dropped_files_ws.update(|dropped_files| {dropped_files.remove(&dragged);});
                            remove_file_ws.set(Some(dragged));
                        }
                    }
                >
                </div>
            </div>
        </div>
    }
}
