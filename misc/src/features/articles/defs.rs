use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ArticleList = Vec<Article>;
pub type ArticleId = String;
pub type ArticleContent = String;
pub type ArticleTag = String;
pub type ArticleTitle = String;

pub trait ArticleSource {
    fn load_article_content(&self) -> ArticleContent;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelativeLocalArticleSource {
    pub relative_path: String,
}
pub struct LocalArticleSource {
    pub base_path: String,
    pub relative_source: RelativeLocalArticleSource,
}

impl ArticleSource for LocalArticleSource {
    fn load_article_content(&self) -> ArticleContent {
        use std::fs::read_to_string;
        use std::path::Path;
        let base_path = Path::new(&self.base_path);
        let full_path = base_path.join(&self.relative_source.relative_path);
        let result = read_to_string(full_path);
        result.expect("to load article content")
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum ArticleCategory {
    Engineering,
    Life,
    Noop,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Article {
    pub relative_source: RelativeLocalArticleSource,
    pub id: ArticleId,
    pub title: ArticleTitle,
    pub tags: Vec<ArticleTag>,
    pub description: Option<String>,
    pub category: ArticleCategory,
    pub timestamp_info_str: Option<String>, // TEMP to keep track of post timings, refactor later
}

pub struct Articles {
    // order in list depends on order in Vec
    pub ordered_articles: ArticleList,

    pub not_found_article: Article,
    pub id_to_article: HashMap<ArticleId, Article>,
}

impl Articles {
    pub fn get_by_id(&self, article_id: ArticleId) -> &Article {
        self.id_to_article
            .get(&article_id)
            .unwrap_or(&self.not_found_article)
    }
}
