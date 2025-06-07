use crate::features::articles::defs::ArticleCategory;
use crate::features::articles::defs::*;
use leptos::prelude::*;

#[server]
pub async fn get_article(article_id: ArticleId) -> Result<Article, ServerFnError> {
    let articles = Articles::default();
    let article = articles.get_by_id(article_id).clone();
    Ok(article)
}

#[server]
pub async fn get_latest_article_id(
    article_category: Option<ArticleCategory>,
) -> Result<ArticleId, ServerFnError> {
    let articles = Articles::default();
    let article = articles
        .ordered_articles
        .into_iter()
        .filter(|article| {
            if let Some(article_category) = &article_category {
                article.category == *article_category
            } else {
                false
            }
        })
        .next()
        .unwrap_or(articles.not_found_article);
    Ok(article.id.clone())
}

#[server]
pub async fn get_article_content(article_id: ArticleId) -> Result<ArticleContent, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let articles = Articles::default();

    let article = articles.get_by_id(article_id);

    let relative_source = article.relative_source.clone();

    let local_source = LocalArticleSource {
        base_path: get_articles_base_path().into(),
        relative_source,
    };

    let content = local_source.load_article_content();

    Ok(content)
}

pub fn get_articles_base_path() -> String {
    use std::path::PathBuf;
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let mut base = PathBuf::from(&*leptos_options.site_root);
    base.push("static/articles");
    let result = base.to_str().unwrap().to_owned();
    result
}

#[server]
pub async fn get_preload_images_links(take_first: usize) -> Result<Vec<String>, ServerFnError> {
    use std::path::PathBuf;
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let mut base = PathBuf::from(&*leptos_options.site_root);
    base.push("static/articles/photography/article.md");

    use std::fs;
    let contents = fs::read_to_string(base).unwrap();

    use regex::Regex;
    let re = Regex::new(r"(/static/articles/photography/images/[A-Za-z0-9_]+\.jpg)").unwrap();

    let urls = re
        .captures_iter(&contents)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_owned())
        .take(take_first)
        .collect();

    Ok(urls)
}
