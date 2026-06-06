import { test, expect } from "@playwright/test";

// Stable article fixture: engineering article with created_at, written_on, tags, and description all set.
const FIXTURE_ARTICLE = "inventing_a_better_compression_algorithm_for_a_specific_problem";

test.describe("robots.txt", () => {
  test("returns correct content-type and required directives", async ({ request }) => {
    const res = await request.get("/robots.txt");
    expect(res.status()).toBe(200);
    expect(res.headers()["content-type"]).toContain("text/plain");
    const body = await res.text();
    expect(body).toContain("User-agent: *");
    expect(body).toContain("Allow: /");
    expect(body).toContain("Sitemap:");
    expect(body).toContain("/sitemap.xml");
  });
});

test.describe("sitemap.xml", () => {
  test("returns valid XML with known article URLs", async ({ request }) => {
    const res = await request.get("/sitemap.xml");
    expect(res.status()).toBe(200);
    expect(res.headers()["content-type"]).toContain("application/xml");
    const body = await res.text();
    expect(body).toContain('<?xml version="1.0"');
    expect(body).toContain("<urlset");
    expect(body).toContain(`/articles/${FIXTURE_ARTICLE}`);
    expect(body).toContain("/articles/about_site");
  });

  test("all locs share the same base URL (no mixed origins)", async ({ request }) => {
    const res = await request.get("/sitemap.xml");
    const body = await res.text();
    const locs = [...body.matchAll(/<loc>(.*?)<\/loc>/g)].map((m) => m[1]);
    expect(locs.length).toBeGreaterThan(0);
    const origins = new Set(locs.map((u) => new URL(u).origin));
    expect(origins.size).toBe(1);
  });
});

test.describe("homepage", () => {
  test("title is set correctly", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle("Alexander Tokar's Blog");
  });

  test("canonical link present and well-formed", async ({ page }) => {
    await page.goto("/");
    const canonical = page.locator("link[rel='canonical']");
    await expect(canonical).toHaveCount(1);
    const href = await canonical.getAttribute("href");
    expect(href).toMatch(/^https?:\/\/.+\/$/);
  });

  test("canonical origin matches page origin after hydration", async ({ page }) => {
    // Regression: Leptos re-runs components in WASM after SSR. std::env::var always
    // returns Err in WASM, so without the web_sys::window().location().origin() fallback
    // the canonical gets overwritten to a wrong origin on the client.
    await page.goto("/");
    await page.waitForTimeout(500);
    const pageOrigin = new URL(page.url()).origin;
    const href = await page.locator("link[rel='canonical']").getAttribute("href");
    expect(href).toContain(pageOrigin);
  });
});

test.describe("article page", () => {
  test("title matches article (not site default)", async ({ page }) => {
    await page.goto(`/articles/${FIXTURE_ARTICLE}`);
    const title = await page.title();
    expect(title).not.toBe("Alexander Tokar's Blog");
    expect(title.length).toBeGreaterThan(0);
  });

  test("canonical contains the article ID in SSR HTML", async ({ request }) => {
    // Checked via request API (no JS) — leptos_meta manages <link> in the DOM
    // asynchronously after hydration, making browser-based locators unreliable.
    const res = await request.get(`/articles/${FIXTURE_ARTICLE}`);
    const html = await res.text();
    expect(html).toContain('rel="canonical"');
    expect(html).toContain(`/articles/${FIXTURE_ARTICLE}`);
  });

  test("canonical origin matches page origin after hydration", async ({ page }) => {
    // Same Leptos hydration regression as the homepage test — guard for article pages too.
    await page.goto(`/articles/${FIXTURE_ARTICLE}`);
    await page.waitForTimeout(500);
    const pageOrigin = new URL(page.url()).origin;
    const href = await page.locator("link[rel='canonical']").getAttribute("href");
    expect(href).toContain(pageOrigin);
  });

  test("JSON-LD script present with BlogPosting type", async ({ page }) => {
    await page.goto(`/articles/${FIXTURE_ARTICLE}`);
    const script = page.locator('script[type="application/ld+json"]');
    await expect(script).toHaveCount(1);
    const content = await script.textContent();
    expect(content).toContain('"@type":"BlogPosting"');
    expect(content).toContain('"@context":"https://schema.org"');
    expect(content).toContain('"datePublished"');
  });

  test("og:url meta tag present in SSR HTML", async ({ request }) => {
    // Checked in raw HTML (no JS) because og:url is a crawler-facing tag.
    const res = await request.get(`/articles/${FIXTURE_ARTICLE}`);
    const html = await res.text();
    expect(html).toContain('property="og:url"');
    expect(html).toContain(`/articles/${FIXTURE_ARTICLE}`);
  });

  test("article content rendered in SSR HTML (visible to crawlers)", async ({ request }) => {
    // Uses the request API (no JS execution) to replicate what a search engine crawler sees.
    // Regression: content was previously loaded via Resource::new (client-side only),
    // making it invisible to Google. Fixed by switching to Resource::new_blocking.
    const res = await request.get(`/articles/${FIXTURE_ARTICLE}`);
    const html = await res.text();
    expect(html).toContain("markdown-body");
    expect(html.length).toBeGreaterThan(5000);
  });
});
