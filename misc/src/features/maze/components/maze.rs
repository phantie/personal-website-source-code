#![allow(unused)]

use crate::features::maze::*;
use leptos::{logging::log, prelude::*};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone, Debug)]
struct CellState {
    hide: bool,
    visited: bool,
    name: String,
    can_move_to: bool,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CellStateChange {
    Init,
    CellVisited((Pos, Cell)),
}

pub enum InitiallyRevealed {
    One(PaddedPos),
}

pub fn render_arena(m: PaddedMatrix, pos: InitiallyRevealed) -> AnyView {
    let (rs, ws) = signal(CellStateChange::Init);

    let m = Rc::new(m);

    let (click_pos_read, click_pos_write) = signal(None);

    let (mousedown_read, mousedown_write) = signal(false);

    let state_signal_matrix = Rc::new(create_shadow_matrix_with(&m, |pos| {
        let (rowi, coli) = pos;
        let cell = m[rowi][coli].clone();
        let cell_state = CellState {
            hide: true,
            visited: false,
            can_move_to: cell.can_move_to,
            name: cell.name.clone(),
        };
        signal(cell_state)
    }));

    // Effect that handles UI changes to VisitState
    {
        let state_signal_matrix = state_signal_matrix.clone();
        let m = m.clone();

        Effect::new(move |_| {
            let msc = rs.get();
            log!("Handling {msc:?}");

            match msc {
                CellStateChange::CellVisited((pos @ (rowi, coli), cell)) => {
                    let (state_rs, state_ws) = state_signal_matrix[rowi][coli];

                    state_ws.update(|cell| {
                        cell.visited = true;
                    });

                    for (rowi, coli) in std::iter::once(pos).chain(
                        Direction::iter()
                            .into_iter()
                            .map(|d| inc_pos_to_direction(pos, d)),
                    ) {
                        let (rs, ws) = state_signal_matrix[rowi][coli];
                        if rs.read_untracked().hide {
                            ws.update(|cell| {
                                cell.hide = false;
                            });
                        }
                    }
                }
                CellStateChange::Init => match pos {
                    InitiallyRevealed::One(pos) => {
                        ws.set(CellStateChange::CellVisited((
                            pos,
                            pick_pos(&m, pos).clone(),
                        )));
                    }
                },
                _ => (),
            }
        });
    }

    /// Effect that handles position revealing by user
    {
        let state_signal_matrix = state_signal_matrix.clone();
        let m = m.clone();

        Effect::new(move |_| {
            let pos: Option<Pos> = click_pos_read.get();

            if let Some(pos @ (rowi, coli)) = pos {
                log!("Trying visit pos: {:?}", pos);

                let (state_rs, state_ws) = state_signal_matrix[rowi][coli];

                if !state_rs.read_untracked().hide
                    && state_rs.read_untracked().can_move_to
                    && !state_rs.read_untracked().visited
                {
                    state_ws.update(|cell| {
                        ws.set(CellStateChange::CellVisited((
                            pos,
                            pick_pos(&m, pos).clone(),
                        )));
                    });
                }
            }
        });
    }

    let mut maze_html = vec![];

    for (rowi, row) in m.iter().enumerate() {
        let mut row_html_els = vec![];

        for (coli, cell) in row.iter().enumerate() {
            let (state_rs, _) = state_signal_matrix[rowi][coli];

            let cell_name = cell.name.clone();

            row_html_els.push(view! {
                <div
                    class="click-maze-col"
                    on:mousedown=move |_| {
                        click_pos_write.write().replace((rowi, coli));
                    }
                    on:mouseenter=move |_| {
                        if *mousedown_read.read_untracked() {
                            click_pos_write.write().replace((rowi, coli));
                        }
                    }
                    class:visited=move || state_rs.get().visited
                    class:hide=move || state_rs.get().hide
                    class:can_move_to=move || state_rs.get().can_move_to
                    class:cant_move_to=move || !state_rs.get().can_move_to

                >
                    // {cell_name}
                    // {" ("}{rowi}{","}{coli}{")"}
                    // {move || format!(" {:?}", state_rs.get())}
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
        <div
            class="click-maze"
            on:mousedown=move |_| {
                mousedown_write.set(true);
            }
            on:mouseup=move |_| {
                mousedown_write.set(false);
            }
        >{ maze_html }</div>
    }
    .into_any()
}

#[component]
pub fn MazeComponent() -> impl IntoView {
    let m = test_mazes::n0();
    let pos = test_mazes::n0_start();

    let m = pad_matrix(m);
    let pos = pad_position(pos);

    view! {
        { render_arena(m, InitiallyRevealed::One(pos)) }
    }
}
