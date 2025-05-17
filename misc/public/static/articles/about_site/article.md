# A blog by [@phantie on GitHub](https://github.com/phantie/)

> А то как сапожник без сапог

## Source Code on [GitHub](https://github.com/phantie/personal-website-source-code)

### Technology Stack

This is the third generation of my personal website.

- **Elixir (Phoenix)** → **Rust (Yew)** → **Rust (Leptos)**

### Implemented Features

- **Articles**
  - Article reading progress bar
  - Code highlighting
  - Selection from a list of articles
  - Auto anchor creation
  - SSR (Server-side rendering) and CSR (Client-side rendering)
  - HTML meta tags (description, keywords) for SEO

- **Hydration**
  - Fixed stale artifacts serving

### Deployment

- Continuous deployment on *main* branch push
- Deployed as a DigitalOcean App using *misc/Dockerfile*
- Connected to *phantie.dev*
- SSL included

#### Minor Experiments

- [Hidden maze](/experiment/maze)
  - Experiment with Leptos signals  
  - Unlike in React, only the affected cells, not the whole component, are redrawn.
- [DragAndDrop](/experiment/dragndrop)
  - Experiment with JS drag events
  - Implemented drag-and-drop of local files
  - Discarding items using drag-and-drop

### Dev Installation

#### [Install Rust](https://www.rust-lang.org/tools/install)

#### Then follow [Leptos Guide](/misc/readme.md)
