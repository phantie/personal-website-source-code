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
  - Support footnotes
  - Return 404 status code on SSR for not found articles for SEO caching

- **Hydration**
  - Fixed stale artifacts serving and site breaking because of it

- **Artifact Caching**
  - In dev:
    - Optimal caching with Etags
  - In prod and in front of CloudFlare:
    - Optimal caching with Etags for images
    - Yet not working with Etags for .wasm, .js, .css

- **Front page**

- **Header**

### Production Deployment

- Continuous deployment on *main* branch push
- Deployed as a DigitalOcean App using *misc/Dockerfile*
- Connected to *phantie.dev*
- SSL included

### Staging Deployment

- Continuous deployment on *staging* branch push
- Deployed as a DigitalOcean App using *misc/Dockerfile*

### Dev Installation

#### [Install Rust](https://www.rust-lang.org/tools/install)

#### Then follow [Leptos Guide](/misc/readme.md)

#### Upgrade dependencies (optional)

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

- [Hidden maze](https://phantie.dev/experiment/maze)
  - Experiment with Leptos signals
  - Unlike in React, only the affected cells, not the whole component, are redrawn.
- [DragAndDrop](https://phantie.dev/experiment/dragndrop)
  - Experiment with JS drag events
  - Implemented drag-and-drop of local files
  - Discarding items using drag-and-drop
