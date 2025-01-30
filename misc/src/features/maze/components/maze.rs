#![allow(unused)]

use std::rc::Rc;

use crate::features::maze::*;
use leptos::{logging::log, prelude::*};

pub fn render_discovered_matrix(value: &Matrix, pos: Pos) -> AnyView {
    let (clicked_pos, clicked_pos_write) = signal(None);

    let hide_matrix = derive_hide_matrix(value);

    let signal_matrix = Rc::new(create_shadow_matrix_with(value, |pos| {
        // let (rowi, coli) = pos;
        // signal::<Cell>(value[rowi][coli].clone())
        signal::<bool>(false)
    }));

    {
        let signal_matrix = signal_matrix.clone();
        Effect::new(move |_| {
            let pos: Option<Pos> = clicked_pos.get();
            if let Some(pos @ (rowi, coli)) = pos {
                let (r, w) = signal_matrix[rowi][coli];
                w.set(true); // TEMP

                log!("WTF: {:?}", *r.read());
                log!("Clicked pos: {:?}", pos);
            }
        });
    }

    let mut maze_html = vec![];

    for (rowi, row) in value.iter().enumerate() {
        let mut row_html_els = vec![];

        for (coli, cell) in row.iter().enumerate() {
            let hide = hide_matrix[rowi][coli];

            let signal_matrix = signal_matrix.clone();
            let (r, w) = signal_matrix[rowi][coli];

            // log!("{rowi} {coli}");

            let current = (rowi, coli) == pos;

            let cell_name = cell.name.clone();

            let f = view! {
                <div class="click-maze-col" on:click=move |_| {
                    clicked_pos_write.write().replace((rowi, coli));
                }>
                {cell_name}

                {if current {" (current)"} else {""}}
                {if hide {" (hide)"} else {""}}

                {","}{rowi}{","}{coli}{","}
                // <div>{format!("{:?}", r.get())}</div>
                {r}
                </div>
            };

            row_html_els.push(view! {
                {f}
            });

            //
        }

        let row_html = view! {
            <div class="click-maze-row">
                { row_html_els }
            </div>
        };

        maze_html.push(row_html);
    }

    view! { <div class="click-maze">{ maze_html }</div> }.into_any()
}

#[component]
pub fn MazeComponent() -> impl IntoView {
    let m = test_mazes::n0();
    let pos = test_mazes::n0_start();
    let s = MovementState::new(m.clone(), pos);

    view! {
        { render_discovered_matrix(&s.m, s.pos) }
    }
}
