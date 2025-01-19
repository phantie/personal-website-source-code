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
    relative_path: String,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Article {
    pub relative_source: RelativeLocalArticleSource,
    pub id: ArticleId,
    pub title: ArticleTitle,
    pub tags: Vec<ArticleTag>,
}

pub struct Articles {
    // order in list depends on order in Vec
    pub ordered_articles: ArticleList,

    not_found_article: Article,
    id_to_article: HashMap<ArticleId, Article>,
}

fn get_not_found_article() -> Article {
    Article {
        relative_source: RelativeLocalArticleSource {
            relative_path: "not_found/not_found.md".into(),
        },
        id: "not_found".into(),
        title: "Not found".into(),
        tags: vec![],
    }
}

fn get_articles_chronological_order() -> ArticleList {
    vec![
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "about_site/article.md".into(),
            },
            id: "about_site".into(),
            title: "About site".into(),
            tags: vec![],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "animals/article.md".into(),
            },
            id: "animals".into(),
            title: "Animals".into(),
            tags: vec![],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "most_useful_functional_pattern_python/article.md".into(),
            },
            id: "most_useful_functional_pattern_python".into(),
            title: "Most useful functional pattern in Python".into(),
            tags: vec![],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "photography/photography.md".into(),
            },
            id: "photography".into(),
            title: "Photography".into(),
            tags: vec![],
        },
    ]
}

impl Default for Articles {
    fn default() -> Self {
        let not_found_article = get_not_found_article();

        let ordered_articles = get_articles_chronological_order()
            .into_iter()
            .rev()
            .collect::<ArticleList>();

        Self {
            id_to_article: ordered_articles
                .iter()
                .map(|article| (article.id.clone(), article.clone()))
                .collect(),
            not_found_article,
            ordered_articles,
        }
    }
}

impl Articles {
    pub fn get_by_id(&self, article_id: ArticleId) -> &Article {
        self.id_to_article
            .get(&article_id)
            .unwrap_or(&self.not_found_article)
    }
}

enum Currency {
    Euro,
    Dollar,
    Peanuts,
}
