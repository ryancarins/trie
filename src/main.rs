use std::string::ParseError;
use std::{convert::Infallible, str::FromStr};
use std::sync::Arc;

use warp::Filter;
use trie::{naive_trie::NaiveTrie, get_words_as_vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Res {
    pub result: bool,
    pub word: String
}

struct Words {
    words: Vec<String>,
}

impl FromStr for Words {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = Words { words: Vec::new() };
        for word in s.split(",") {
            words.words.push(word.to_string());
        }

        Ok(words)
    }
}

fn with_trie(trie: Arc<NaiveTrie>) -> impl Filter<Extract = (Arc<NaiveTrie>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || trie.clone())
}

async fn contains(words: Words, trie: Arc<NaiveTrie>) -> Result<impl warp::Reply, Infallible>   {
    let mut results: Vec<Res> = Vec::new();
    for word in words.words {
        results.push(Res{result: trie.contains(&word), word});
    }
    return Ok(
        warp::reply::json(
            &results
        )
    );
}

#[tokio::main]
async fn main() {
    let mut trie = NaiveTrie::default();
    let words = get_words_as_vec();

    for word in &words {
        trie.insert(word);
    }

    let arc_trie = Arc::new(trie);

    let is_word = warp::get()
        .and(warp::path("is_word"))
        .and(warp::path::param::<Words>())
        .and(warp::path::end())
        .and(with_trie(arc_trie))
        .and_then(contains);

    let routes = is_word;
 
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
