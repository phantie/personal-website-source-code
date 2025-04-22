# A blog of [@phantie on Github](https://github.com/phantie/)

> А то как сапожник без сапог

## Source code on [Github](https://github.com/phantie/leptos-wsite)

### Technology Stack

This is a 3rd generation of personal website.

- **Elixir (Phoenix)** → **Rust (Yew)** → **Rust (Leptos)**

### Implemented Features:
- **Articles**
    - Selection
    - Auto anchor creation
    - Code highlighting
    - SSR (Server-side rendering) and CSR (Client-side rendering)
    - HTML Meta tag (description, keywords) for CEO
    - Article reading progress bar

### Deployment

- Continuous deployment on *main* branch push
- Deployed as DigitalOcean App using *misc/Dockerfile*
- Connected to *phantie.dev*
- SSL included

#### Minor experiments
- [Hidden maze](/experiment/maze)
    - Experiment with Leptos signals
    - Unlike in React, only affected cells, not the whole component gets redrawn.
- [DragAndDrop](/experiment/dragndrop)
    - Experiment with JS drag events
    - Implemented drag-and-drop of local files
    - Discarding of items using drag-and-drop
