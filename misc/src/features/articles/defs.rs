use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type ArticleId = String;
pub type ArticleContent = String;
pub type ArticleTag = String;
pub type ArticleTitle = String;

pub trait ArticleSource {
    fn load_article_content(&self) -> ArticleContent;
}

#[derive(Clone)]
pub struct RelativeLocalArticleSource {
    relative_path: String,
}
pub struct LocalArticleSource {
    pub base_path: String,
    pub relative_source: RelativeLocalArticleSource,
}

impl ArticleSource for LocalArticleSource {
    fn load_article_content(&self) -> ArticleContent {
        use std::fs::read_to_string;
        use std::path::{Path, PathBuf};
        let base_path = Path::new(&self.base_path);
        let full_path = base_path.join(&self.relative_source.relative_path);
        let result = read_to_string(full_path);
        result.expect("to load article content")
    }
}

pub struct RelativeLocalArticleSources {
    first: RelativeLocalArticleSource,
    not_found: RelativeLocalArticleSource,
}

impl Default for RelativeLocalArticleSources {
    fn default() -> Self {
        Self {
            first: RelativeLocalArticleSource {
                relative_path: "first.md".into(),
            },
            not_found: RelativeLocalArticleSource {
                relative_path: "not_found.md".into(),
            },
        }
    }
}

impl RelativeLocalArticleSources {
    pub fn get_by_id(&self, article_id: &ArticleId) -> &RelativeLocalArticleSource {
        match article_id.as_ref() {
            "first" => &self.first,
            _ => &self.not_found,
        }
    }
}

#[derive(Clone)]
struct Article {
    relative_source: RelativeLocalArticleSource,
    id: ArticleId,
    title: ArticleTitle,
    tags: Vec<ArticleTag>,
}

struct Articles {
    pub inner: Vec<Article>,

    not_found: Article,
    id_to_article: HashMap<ArticleId, Article>,
}

impl Default for Articles {
    fn default() -> Self {
        let not_found = Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "not_found.md".into(),
            },
            id: "not_found".into(),
            title: "Not found".into(),
            tags: vec![],
        };

        let articles = vec![
            Article {
                relative_source: RelativeLocalArticleSource {
                    relative_path: "first.md".into(),
                },
                id: "first".into(),
                title: "First article".into(),
                tags: vec![],
            },
            not_found.clone(),
        ];

        Self {
            id_to_article: articles
                .iter()
                .map(|article| (article.id.clone(), article.clone()))
                .collect(),
            not_found,
            inner: articles,
        }
    }
}

impl Articles {
    pub fn get_by_id(&self, article_id: ArticleId) -> &Article {
        self.id_to_article
            .get(&article_id)
            .unwrap_or(&self.not_found)
    }
}
