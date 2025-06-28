use crate::features::articles::defs::*;

pub fn get_articles_chronological_order() -> ArticleList {
    vec![
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "about_site/article.md".into(),
            },
            id: "about_site".into(),
            title: "About site (updated 10/06/2025)".into(),
            description: None,
            tags: vec!["about site".into()],
            category: ArticleCategory::Engineering,
            timestamp_info_str: Some("updated_on:10/06/25".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/most_useful_functional_pattern_python/article.md".into(),
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
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/concept_of_more_useful_types/article.md".into(),
            },
            id: "concept_of_more_useful_types".into(),
            title: "Concept of more useful types".into(),
            description: Some("Identifying more useful types would save you from later refactoring, positively impact performance by cutting indirection and aid in writing reusable code.".into()),
            tags: vec![
                "concept".into(),
                "types".into(),
                "functional programming".into(),
                "python".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/multilayer_cache/article.md".into(),
            },
            id: "multilayer_cache".into(),
            title: "Nano framework for implementing multilayered caching".into(),
            description: Some("Flexible multilayer caching implemented functionally".into()),
            tags: vec![
                "caching".into(),
                "framework".into(),
                "pattern".into(),
                "functional programming".into(),
                "python".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/event_processing/article.md".into(),
            },
            id: "event_processing".into(),
            title: "Event processing".into(),
            description: Some("Explore indemportency and event processing for designing resilient, retry-friendly processes ensuring consistency in complex systems.".into()),
            tags: vec![
                "event processing".into(),
                "idempotency".into(),
                "operations".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/io_bound_parallel_processing_in_python/article.md".into(),
            },
            id: "io_bound_parallel_processing_in_python".into(),
            title: "I/O Bound Parallel Processing in Python".into(),
            description: Some("Handle I/O-heavy work efficiently in Python using asyncio. Explore producer-consumer pattern and solutions to common problems for robust concurrent processing.".into()),
            tags: vec![
                "parallel processing".into(),
                "asyncio".into(),
                "queue".into(),
                "python".into(),
                "producer-consumer".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/vscode_extensions_and_settings_regardless_of_the_programming_language/article.md".into(),
            },
            id: "vscode_extensions_and_settings_regardless_of_the_programming_language".into(),
            title: "My VSCode extensions and settings recommendations, regardless of the programming language (updated 15/06/2025)".into(),
            description: Some("My VSCode extensions and settings recommendations, regardless of the programming language.".into()),
            tags: vec![
                "vscode".into(),
                "extensions".into(),
                "settings".into(),
                "preferences".into(),
                "recommendations".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "poems/a_poem_about_absolute_uncertainty/article.md".into(),
            },
            id: "a_poem_about_absolute_uncertainty".into(),
            title: "A poem about absolute uncertainty".into(),
            description: Some("A poem named 'A poem about absolute uncertainty'".into()),
            tags: vec![
                "original poem".into(),
                "alexander tokar".into(),
                "uncertainty".into(),
                "sean rowe".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("written_on:08/04/24".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "photography/article.md".into(),
            },
            id: "photography".into(),
            title: "Photography (updated 28/06/2025)".into(),
            description: None,
            tags: vec!["photography".into()],
            category: ArticleCategory::Life,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "life/white_roses_spread_on_the_ground/article.md".into(),
            },
            id: "white_roses_spread_on_the_ground".into(),
            title: "White roses spread on the ground".into(),
            description: Some("Thoughts on white roses spread on the ground occurence".into()),
            tags: vec![
                "thoughts".into(),
                "roses".into(),
                "uncertainty".into(),
                "rejection".into(),
                "spite".into(),
                "flowers".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: None,
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "life/unflourished/article.md".into(),
            },
            id: "unflourished".into(),
            title: "Unflourished".into(),
            description: Some("Unflorished flower".into()),
            tags: vec![
                "life".into(),
                "hopes".into(),
                "nature".into(),
                "flowers".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("created_at:30/05/25;".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "poems/farewell_father/article.md".into(),
            },
            id: "farewell_father".into(),
            title: "Farewell, Father".into(),
            description: Some("A poem named 'Farewell, Father'".into()),
            tags: vec![
                "original poem".into(),
                "alexander tokar".into(),
                "late father".into(),
                "war against Ukraine".into(),
                "with photos".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("created_at:05/06/25;written_on:07/08/23".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "poems/the_list/article.md".into(),
            },
            id: "the_list".into(),
            title: "The List".into(),
            description: Some("A poem named 'The List'".into()),
            tags: vec![
                "original poem".into(),
                "alexander tokar".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("created_at:05/06/25;written_on:05/06/25".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "poems/why_are_you_like_this/article.md".into(),
            },
            id: "why_are_you_like_this".into(),
            title: "Why are you like this?".into(),
            description: Some("A poem named 'Why are you like this?'".into()),
            tags: vec![
                "original poem".into(),
                "alexander tokar".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("created_at:11/06/25;written_on:10-11/06/25".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "engineering/inventing_a_better_compression_algorithm_for_a_specific_problem/article.md".into(),
            },
            id: "inventing_a_better_compression_algorithm_for_a_specific_problem".into(),
            title: "Inventing a Better Compression Algorithm for a Specific Problem".into(),
            description: Some("An article about how to leverage domain knowledge to invent spectacular data compression algorithms".into()),
            tags: vec![
                "domain knowledge".into(),
                "compression algorithms".into(),
                "problem solving".into(),
                "bit operations".into(),
                "rust lang".into(),
            ],
            category: ArticleCategory::Engineering,
            timestamp_info_str: Some("created_at:14/06/25;written_on:13-14/06/25".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "poems/look_down_on_those_lights/article.md".into(),
            },
            id: "look_down_on_those_lights".into(),
            title: "Look down on those lights".into(),
            description: Some("A poem named 'Look down on those lights'".into()),
            tags: vec![
                "original poem".into(),
                "alexander tokar".into(),
            ],
            category: ArticleCategory::Life,
            timestamp_info_str: Some("created_at:20/06/25;written_on:19-20/06/25".into()),
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "life/to_my_sister_and_other_graduates/article.md".into(),
            },
            id: "to_my_sister_and_other_graduates".into(),
            title: "To my sister and other graduates".into(),
            description: None,
            tags: vec![
                "photography".into(),
                "best wishes".into(),
                ],
                category: ArticleCategory::Life,
                timestamp_info_str: Some("created_at:22/06/25;written_on:22/06/25".into()),
            },
            Article {
                relative_source: RelativeLocalArticleSource {
                    relative_path: "poems/when_days_become_shorter/article.md".into(),
                },
                id: "when_days_become_shorter".into(),
                title: "When days become shorter".into(),
                description: Some("A poem named 'When days become shorter'".into()),
                tags: vec![
                    "original poem".into(),
                    "alexander tokar".into(),
                ],
                category: ArticleCategory::Life,
                timestamp_info_str: Some("created_at:25/06/25;written_on:25/06/25".into()),
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

pub const NOT_FOUND_ARTICLE_ID: &str = "not_found";

pub fn get_not_found_article() -> Article {
    Article {
        relative_source: RelativeLocalArticleSource {
            relative_path: "not_found/not_found.md".into(),
        },
        id: NOT_FOUND_ARTICLE_ID.into(),
        title: "Not found".into(),
        description: Some("Sentinel article for 404".into()),
        tags: vec![],
        category: ArticleCategory::Noop,
        timestamp_info_str: None,
    }
}
