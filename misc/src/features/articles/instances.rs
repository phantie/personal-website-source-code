use crate::features::articles::defs::*;

pub fn get_not_found_article() -> Article {
    Article {
        relative_source: RelativeLocalArticleSource {
            relative_path: "not_found/not_found.md".into(),
        },
        id: "not_found".into(),
        title: "Not found".into(),
        description: Some("Sentinel article for 404".into()),
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
            description: None,
            tags: vec!["about site".into()],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "photography/photography.md".into(),
            },
            id: "photography".into(),
            title: "Photography".into(),
            description: None,
            tags: vec!["photography".into()],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "most_useful_functional_pattern_python/article.md".into(),
            },
            id: "most_useful_functional_pattern_python".into(),
            title: "Most useful functional pattern in Python".into(),
            description: Some(
                "Most useful functional pattern in Python about disjoined unions with pydantic"
                    .into(),
            ),
            tags: vec![
                "disjoined union".into(),
                "pattern".into(),
                "functional programming".into(),
                "python".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "concept_of_more_useful_types/article.md".into(),
            },
            id: "concept_of_more_useful_types".into(),
            title: "Concept of more useful types".into(),
            description: None,
            tags: vec![
                "concept".into(),
                "types".into(),
                "functional programming".into(),
                "python".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "multilayer_cache/article.md".into(),
            },
            id: "multilayer_cache".into(),
            title: "Nano framework for implementing multilayered caching".into(),
            description: None,
            tags: vec![
                "caching".into(),
                "framework".into(),
                "pattern".into(),
                "functional programming".into(),
                "python".into(),
            ],
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
