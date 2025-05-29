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
  - Fixed stale artifacts serving and site breaking because of it

- **Artifact Caching**
  - In dev:
    - Optimal caching with Etags
  - In prod and in front of CloudFlare:
    - Optimal caching with Etags for images
    - Yet not working with Etags for .wasm, .js, .css

### Features planned to be implemented

- Finish Artifact Caching
- SEO site optimize site description
- Add categories to articles
- Add bio articles with links
- Add a placeholder for still loading images
- Implement articles preload
  - Of the next few
  - Of the hovered over for a while
- Implement and connect dynamic content server (for images/articles) for quicker article modifications

### Deployment

- Continuous deployment on *main* branch push
- Deployed as a DigitalOcean App using *misc/Dockerfile*
- Connected to *phantie.dev*
- SSL included

### Dev Installation

#### [Install Rust](https://www.rust-lang.org/tools/install)

#### Then follow [Leptos Guide](/misc/readme.md)

#### Upgrade dependencies

##### Install cargo-edit

```bash
cargo install cargo-edit
```

##### Updgrade dependencies via `cargo upgrade`

```bash
cargo upgrade --verbose
```

or

> Might break the build.
> But after running the next command after 4 months since I started developing this project, nothing has broken.

```bash
cargo upgrade --verbose --incompatible
```

#### Minor Experiments

- [Hidden maze](/experiment/maze)
  - Experiment with Leptos signals
  - Unlike in React, only the affected cells, not the whole component, are redrawn.
- [DragAndDrop](/experiment/dragndrop)
  - Experiment with JS drag events
  - Implemented drag-and-drop of local files
  - Discarding items using drag-and-drop
