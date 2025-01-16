use std::path::{Path, PathBuf};

pub type ArticleId = String;
pub type ArticleContent = String;

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
