#![allow(unused)]

use crate::features::maze::*;
use leptos::{logging::log, prelude::*};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone, Debug)]
struct CellState {
    hide: bool,
}

pub fn render_arena(mut s: MovementState) -> AnyView {
    let (rs, ws) = signal(None);
    s.subscribe(ws);

    let value = s.m.clone();
    let pos = s.pos;

    let (clicked_pos, clicked_pos_write) = signal(None);

    let movement_signal_matrix = Rc::new(create_shadow_matrix_with(&value, |pos| {
        let (rowi, coli) = pos;
        let cell = value[rowi][coli].clone();
        signal(cell)
    }));

    let state_signal_matrix = Rc::new(create_shadow_matrix_with(&value, |pos| {
        let (rowi, coli) = pos;
        let cell = value[rowi][coli].clone();
        let cell_state = CellState { hide: true };
        signal(cell_state)
    }));

    // Effect that handles UI changes to MovementState
    {
        let movement_signal_matrix = movement_signal_matrix.clone();
        let state_signal_matrix = state_signal_matrix.clone();

        Effect::new(move |_| {
            let msc = rs.get();
            log!("{msc:?}");

            match msc {
                Some(MovementStateChange::CellVisited((pos @ (rowi, coli), cell))) => {
                    let (rs, ws) = movement_signal_matrix[rowi][coli];
                    ws.set(cell);

                    for d in Direction::iter() {
                        let (rowi, coli) = inc_pos_to_direction(pos, d);
                        let (rs, ws) = state_signal_matrix[rowi][coli];
                        if rs.read_untracked().hide {
                            ws.update(|cell| {
                                cell.hide = false;
                            });
                        }
                    }
                }
                _ => (),
            }
        });
    }

    /// Effect that handles cell clicking
    {
        let movement_signal_matrix = movement_signal_matrix.clone();
        Effect::new(move |_| {
            let pos: Option<Pos> = clicked_pos.get();

            if let Some(pos @ (rowi, coli)) = pos {
                log!("Clicked pos: {:?}", pos);

                // apply changes to MovementState
                if s.can_visit_cell(pos) {
                    s.visit_cell(pos);
                }
            }
        });
    }

    let mut maze_html = vec![];

    for (rowi, row) in value.iter().enumerate() {
        let mut row_html_els = vec![];

        for (coli, cell) in row.iter().enumerate() {
            let (movement_rs, _) = movement_signal_matrix[rowi][coli];
            let (state_rs, _) = state_signal_matrix[rowi][coli];

            let current = (rowi, coli) == pos;

            let cell_name = cell.name.clone();

            row_html_els.push(view! {
                <div
                    class="click-maze-col"
                    on:click=move |_| {
                        clicked_pos_write.write().replace((rowi, coli));
                    }
                    class:visited=move || movement_rs.get().visited
                    class:hide=move || state_rs.get().hide

                >
                    {cell_name}
                    {if current {" (current)"} else {""}}
                    {move || if state_rs.get().hide {" (hide)"} else {""}}
                    // {" ("}{rowi}{","}{coli}{")"}
                    {move || format!(" {:?}", state_rs.get())}
                    {move || format!(" {:?}", movement_rs.get())}
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

    let m = pad_matrix(m);
    let pos = pad_position(pos);

    let s = MovementState::new_from_padded(m.clone(), pos);

    view! {
        { render_arena(s) }
    }
}
