use crate::features::articles::defs::*;

pub fn get_not_found_article() -> Article {
    Article {
        relative_source: RelativeLocalArticleSource {
            relative_path: "not_found/not_found.md".into(),
        },
        id: "not_found".into(),
        title: "Not found".into(),
        tags: vec![],
    }
}

pub fn get_articles_chronological_order() -> ArticleList {
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
