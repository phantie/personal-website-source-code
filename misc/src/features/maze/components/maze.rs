use leptos::prelude::*;

#[component]
pub fn MazeComponent() -> impl IntoView {
    let _blocks = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];

    view! {
        <div class="click-maze">

            <div class="click-maze-row">
                <div class="click-maze-col">
                1
                </div>
                <div class="click-maze-col">
                2
                </div>
            </div>
            <div class="click-maze-row">
                <div class="click-maze-col">
                3
                </div>
                <div class="click-maze-col">
                4
                </div>
            </div>

        </div>

    }
}
