use clap::{App, Arg};
use reqwest;
use serde_json::Value;
use std::collections::{HashSet};

// Function to make GET requests to ConceptNet API
async fn get_conceptnet_edges(
    word: &str,
    relation: &str,
    language: &str,
    limit: u32,
) -> Result<Value, reqwest::Error> {
    let url = format!(
        "http://api.conceptnet.io/c/{}/{}?rel={}&limit={}",
        language, word, relation, limit
    );
    let response = reqwest::get(&url).await?;
    Ok(response.json::<Value>().await?)
}

// Function to extract words from the API response
fn extract_words(data: &Value, language: &str) -> Vec<String> {
    let mut words = HashSet::new();
    if let Some(edges) = data["edges"].as_array() {
        for edge in edges {
            if let Some(start) = edge["start"].as_object() {
                if start
                    .get("language")
                    .map(|l| l.as_str() == Some(language))
                    .unwrap_or(false)
                {
                    if let Some(label) = start["label"].as_str() {
                        words.insert(label.to_string());
                    }
                }
            }
            if let Some(end) = edge["end"].as_object() {
                if end
                    .get("language")
                    .map(|l| l.as_str() == Some(language))
                    .unwrap_or(false)
                {
                    if let Some(label) = end["label"].as_str() {
                        words.insert(label.to_string());
                    }
                }
            }
        }
    }
    words.into_iter().collect()
}

#[tokio::main]
async fn main() {
    // let relations = HashMap::from([
    //     ("typen", "/r/IsA"),
    //     ("ableitung", "/r/DerivedFrom"),
    //     ("synonym", "/r/Synonym"),
    //     ("form", "/r/FormOf"),
    //     ("unterschied-zu", "/r/DistinctFrom"),
    //     ("antonym", "/r/Antonym"),
    // ]);

    let matches = App::new("ConceptNet Query Tool")
        .arg(
            Arg::new("language")
                .short('l')
                .takes_value(true)
                .default_value("de"),
        )
        .arg(Arg::new("word").short('w').takes_value(true).required(true))
        .arg(
            Arg::new("relation")
                .short('r')
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("limit_api")
                .short('a')
                .takes_value(true)
                .default_value("1000"),
        )
        .arg(
            Arg::new("limit")
                .short('m')
                .takes_value(true)
                .default_value("50"),
        )
        .after_help("
typen........: /r/IsA
ableitung....: /r/DerivedFrom
synonym......: /r/Synonym
form.........: /r/FormOf
unterschied..: /r/DistinctFrom
antonym......: /r/Antonym
")
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let word = matches.value_of("word").unwrap();
    let relation = matches.value_of("relation").unwrap();
    let limit_api: u32 = matches.value_of("limit_api").unwrap().parse().unwrap();
    let limit: usize = matches.value_of("limit").unwrap().parse().unwrap();

    match get_conceptnet_edges(word, relation, language, limit_api).await {
        Ok(data) => {
            let words = extract_words(&data, language);
            for word in words.into_iter().take(limit) {
                println!("{}", word);
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
