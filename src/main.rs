use std::convert::Infallible;
use std::sync::Arc;

use warp::Filter;
use trie::{trimmed_vec_trie::TrimmedVecTrie, get_words_as_vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Res {
    pub result: bool,
}

fn with_trie(trie: Arc<TrimmedVecTrie>) -> impl Filter<Extract = (Arc<TrimmedVecTrie>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || trie.clone())
}

async fn contains(word: String, trie: Arc<TrimmedVecTrie>) -> Result<impl warp::Reply, Infallible>   {
    let result: Res = Res{result: trie.contains(&word)};
    return Ok(
        warp::reply::json(
            &result
        )
    );
}

#[tokio::main]
async fn main() {
    let mut trie = TrimmedVecTrie::default();
    let words = get_words_as_vec();

    for word in &words {
        trie.insert(word);
    }

    let arc_trie = Arc::new(trie);

    let is_word = warp::get()
        .and(warp::path("is_word"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(with_trie(arc_trie))
        .and_then(contains);

    let routes = is_word;
 
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
