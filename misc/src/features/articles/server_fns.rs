use crate::features::articles::defs::*;
use leptos::prelude::*;

#[server]
pub async fn get_article(article_id: ArticleId) -> Result<Article, ServerFnError> {
    let articles = Articles::default();
    let article = articles.get_by_id(article_id).clone();
    Ok(article)
}

#[server]
pub async fn get_any_article_id() -> Result<ArticleId, ServerFnError> {
    let articles = Articles::default();
    let article = articles.ordered_articles.first().unwrap();
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

fn get_articles_base_path() -> &'static str {
    use std::path::PathBuf;
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let mut base = PathBuf::from(&*leptos_options.site_root);
    base.push("static/articles");
    let result = base.to_str().unwrap().to_owned().leak();
    result
}
