import { test, expect } from "@playwright/test";

test.describe("homepage", () => {
  test("shows section links for all three categories", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("a[href='/articles?category=engineering']")).toBeVisible();
    await expect(page.locator("a[href='/articles?category=poetry']")).toBeVisible();
    await expect(page.locator("a[href='/articles?category=life']")).toBeVisible();
  });
});

test.describe("article list", () => {
  test("engineering category renders article links", async ({ page }) => {
    await page.goto("/articles?category=engineering");
    const items = page.locator(".articles-list-item");
    await expect(items.first()).toBeVisible();
    const count = await items.count();
    expect(count).toBeGreaterThan(0);
  });

  test("unknown category redirects to home", async ({ page }) => {
    await page.goto("/articles?category=this_is_not_a_category");
    await expect(page).toHaveURL("/");
  });
});

test.describe("article page", () => {
  const ARTICLE = "inventing_a_better_compression_algorithm_for_a_specific_problem";

  test("unknown article ID redirects to home", async ({ page }) => {
    await page.goto("/articles/this_article_does_not_exist_xyzabc");
    await expect(page).toHaveURL("/");
  });

  test("header has home link pointing to /", async ({ page }) => {
    await page.goto(`/articles/${ARTICLE}`);
    const home = page.locator("a.home-link");
    await expect(home).toBeVisible();
    expect(await home.getAttribute("href")).toBe("/");
  });

  test("reading progress bar is in the DOM", async ({ page }) => {
    // Bar starts at width:0, so toBeVisible() fails; toBeAttached() confirms DOM presence.
    await page.goto(`/articles/${ARTICLE}`);
    await expect(page.locator(".article-reading-progress")).toBeAttached();
  });
});
