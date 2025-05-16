use crate::features::articles::defs::*;

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
            description: Some("When designing a system, it’s often helpful to choose the most broadly applicable representation of your data—one that can be reached easily from multiple sources. By selecting a more general “useful” type, you reduce unnecessary conversions and ensure your design remains flexible and performant.".into()),
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
            description: Some("An overview of a flexible, multi-layer caching approach in Python, where each cache layer can depend on another in a chain or tree-like structure. It illustrates how data moves from a source (e.g., an S3 bucket) through in-memory caches—such as a file cache and a parsed file cache—so that once a value is locally cached, it can be quickly retrieved without repeatedly contacting upstream layers. The article includes a step-by-step example featuring a two-layer implementation, discusses common pitfalls of multi-layer caching (like code complexity and tight coupling), and introduces the “multilayer_cache” library, offering both synchronous and asynchronous caching solutions.".into()),
            tags: vec![
                "caching".into(),
                "framework".into(),
                "pattern".into(),
                "functional programming".into(),
                "python".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "event_processing/article.md".into(),
            },
            id: "event_processing".into(),
            title: "Event processing".into(),
            description: Some("This article explores how events transition between states and the inherent need for repeatable (idempotent) operations. It contrasts pure calculations (which leave no trace) with side effects (which alter state) and argues that, even when outcomes are partial or uncertain, designing resilient, retry-friendly processes ensures consistency in complex systems.".into()),
            tags: vec![
                "event processing".into(),
                "idempotency".into(),
                "operations".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "io_bound_parallel_processing_in_python/article.md".into(),
            },
            id: "io_bound_parallel_processing_in_python".into(),
            title: "I/O Bound Parallel Processing in Python".into(),
            description: Some("This article shows how to handle I/O-heavy work efficiently in Python using asyncio. It covers setting up async functions, using asyncio.gather for parallelism, and managing large task loads with producer-consumer queues. Practical examples illustrate handling timeouts, exceptions, and scaling concurrency while keeping your code robust and maintainable.".into()),
            tags: vec![
                "parallel processing".into(),
                "asyncio".into(),
                "queue".into(),
                "python".into(),
                "producer-consumer".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "vscode_extensions_and_settings_regardless_of_the_programming_language/article.md".into(),
            },
            id: "vscode_extensions_and_settings_regardless_of_the_programming_language".into(),
            title: "My VSCode extensions and settings recommendations, regardless of the programming language".into(),
            description: Some("This article recommends extensions and settings for VSCode regardless of the programming language.".into()),
            tags: vec![
                "vscode".into(),
                "extensions".into(),
                "settings".into(),
                "preferences".into(),
                "recommendations".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "a_poem_about_absolute_uncertainty/article.md".into(),
            },
            id: "a_poem_about_absolute_uncertainty".into(),
            title: "A poem about absolute uncertainty".into(),
            description: Some("A poem about absolute uncertainty.".into()),
            tags: vec![
                "poem".into(),
                "original".into(),
                "alexander tokar".into(),
                "uncertainty".into(),
                "sean rowe".into(),
            ],
        },
        Article {
            relative_source: RelativeLocalArticleSource {
                relative_path: "white_roses_spread_on_the_ground/article.md".into(),
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
