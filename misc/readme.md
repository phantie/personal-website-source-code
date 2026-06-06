# misc

Leptos + Axum SSR app powering [phantie.dev](https://phantie.dev).

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing

### Unit tests

No server needed. Tests run on the host target with the `ssr` feature:

```bash
cargo test --features ssr
```

**What is covered:**

- `features::articles::defs` — `ArticleCategory`
  - `try_from` maps all valid strings (`engineering`, `life`, `poetry`, `noop`) to the correct variant
  - `try_from` returns `Err` for unknown or wrong-case strings
  - `to_string` → `try_from` roundtrip holds for every variant

- `features::articles::instances` — article registry integrity
  - List is non-empty
  - All article IDs are unique
  - No article has an empty `id`, `title`, or `relative_path`
  - `written_on` is never a later date than `created_at`
  - `get_not_found_article()` carries the sentinel ID `"not_found"`
  - `Articles::get_by_id` returns the correct article for a known ID
  - `Articles::get_by_id` returns the not-found article for an unknown ID

- `features::articles::components::article` — `build_article_json_ld`
  - Required JSON-LD fields are present (`@context`, `@type`, `headline`, `description`, `url`, `author`)
  - Special characters in title/description are escaped (`"`, `\`, newlines)
  - Dates are formatted as ISO 8601 (`YYYY-MM-DD`)
  - `datePublished` / `dateCreated` fields absent when dates are `None`
  - Tags serialised as a JSON array under `keywords`; field omitted when no tags

### End-to-end tests (Playwright)

Requires the dev server to be running:

```bash
cd misc && cargo leptos watch
```

Then in a separate terminal:

```bash
# First time only
cd misc/end2end && npm install && npx playwright install

npx playwright test --project=chromium
```

To run against a different origin (e.g. production):

```bash
BASE_URL=https://phantie.dev npx playwright test --project=chromium
```

**What is covered** (`end2end/tests/seo.spec.ts`):

- `robots.txt` — correct `Content-Type`, `User-agent: *`, `Allow: /`, `Sitemap:` line present
- `sitemap.xml` — correct `Content-Type`, valid XML structure, known article IDs present, all `<loc>` entries share a single origin (no mixed localhost/prod URLs)
- Homepage — correct `<title>`, canonical `<link>` present and well-formed, canonical origin matches the page origin after client-side hydration
- Article page — title differs from site default, canonical contains article ID in SSR HTML, canonical origin matches page origin after hydration, `<script type="application/ld+json">` present with `BlogPosting` type, `og:url` present in SSR HTML, article content present in SSR HTML (not dependent on JavaScript)

**What is covered** (`end2end/tests/app.spec.ts`):

- Homepage — all three category section links (Engineering, Poetry, Life) are visible
- Article list — engineering category renders at least one article link; unknown category redirects to home
- Article page — unknown article ID redirects to home; header home link points to `/`; reading progress bar is present in the DOM

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
misc
site/
```

Set the following environment variables (updating for your project as needed):

```sh
export LEPTOS_OUTPUT_NAME="misc"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
