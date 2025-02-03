# This is a 3rd generation of personal website.

> А то как сапожник без сапог

### Technology Stack
- **Elixir (Phoenix)** → **Rust (Yew)** → **Rust (Leptos)**

### Implemented Features:
- **Articles**
    - Selection
    - Auto anchor creation
    - Code highlighting
    - SSR (Server-side rendering) and CSR (Client-side rendering)
    - HTML Meta tag (description, keywords)
- **Experiments**
    - [Hidden maze](/experiment/maze)
        - Experiment with Leptos signals
        - Unlike in React, only affected cells, not the whole component gets redrawn.
    - [DragAndDrop](/experiment/dragndrop)
        - Experiment with JS drag events
        - Implemented drag-and-drop of local files
        - Discarding of items using drag-and-drop


### Deployment

- Continuous deployment on *main* branch push
- Deployed as DigitalOcean App using *misc/Dockerfile*
- Connected to *phantie.dev*
- SSL included
