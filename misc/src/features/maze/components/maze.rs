#![allow(unused)]

use crate::features::maze::*;
use leptos::{logging::log, prelude::*};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone, Debug)]
struct CellState {
    inner: Cell,
}

pub fn render_arena(mut s: MovementState) -> AnyView {
    let (rs, ws) = signal(None);
    s.subscribe(ws);

    let value = s.m.clone();
    let pos = s.pos;

    let (clicked_pos, clicked_pos_write) = signal(None);

    let hide_matrix = derive_hide_matrix(&value);

    let signal_matrix = Rc::new(create_shadow_matrix_with(&value, |pos| {
        let (rowi, coli) = pos;
        let cell = value[rowi][coli].clone();
        signal(cell)
    }));

    let state_matrix = Rc::new(create_shadow_matrix_with(&value, |pos| {
        let (rowi, coli) = pos;
        let cell = value[rowi][coli].clone();
        CellState { inner: cell }
    }));

    // Effect that handles UI changes to MovementState
    {
        let signal_matrix = signal_matrix.clone();

        Effect::new(move |_| {
            let msc = rs.get();
            log!("{msc:?}");

            match msc {
                Some(MovementStateChange::CellChanged(((rowi, coli), cell))) => {
                    let (rs, ws) = signal_matrix[rowi][coli];
                    ws.set(cell);
                }
                _ => (),
            }
        });
    }

    /// Effect that handles cell clicking
    {
        let signal_matrix = signal_matrix.clone();
        Effect::new(move |_| {
            let pos: Option<Pos> = clicked_pos.get();
            if let Some(pos @ (rowi, coli)) = pos {
                log!("Clicked pos: {:?}", pos);

                // apply changes to MovementState
                match web::can_move_to_cells(&s)
                    .iter()
                    .find(|(_pos, d)| _pos == &pos)
                {
                    Some((_pos, d)) => {
                        s.move_to_direction_once(*d);
                    }
                    None => (),
                }
            }
        });
    }

    let mut maze_html = vec![];

    for (rowi, row) in value.iter().enumerate() {
        let mut row_html_els = vec![];

        for (coli, cell) in row.iter().enumerate() {
            let hide = hide_matrix[rowi][coli];

            let (rs, ws) = signal_matrix[rowi][coli];

            let current = (rowi, coli) == pos;

            let cell_name = cell.name.clone();

            row_html_els.push(view! {
                <div
                    class="click-maze-col"
                    on:click=move |_| {
                        clicked_pos_write.write().replace((rowi, coli));
                    }
                    class:visited=move || rs.get().visited

                >
                    {cell_name}
                    {if current {" (current)"} else {""}}
                    {if hide {" (hide)"} else {""}}
                    // {" ("}{rowi}{","}{coli}{")"}
                    {move || format!(" {:?}", rs.get())}
                </div>
            });
        }

        let row_html = view! {
            <div class="click-maze-row">
                { row_html_els }
            </div>
        };

        maze_html.push(row_html);
    }

    view! {
        <div class="click-maze">{ maze_html }</div>
    }
    .into_any()
}

#[component]
pub fn MazeComponent() -> impl IntoView {
    let m = test_mazes::n0();
    let pos = test_mazes::n0_start();
    let s = MovementState::new(m.clone(), pos);

    view! {
        { render_arena(s) }
    }
}
