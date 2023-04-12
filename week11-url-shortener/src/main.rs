use std::collections::HashMap;
use std::hash::{Hash, Hasher, SipHasher};
use std::io;
use base62::encode;

// Generate a short alias for the given URL
fn generate_alias(url: &str) -> String {
    let mut hasher = SipHasher::new_with_keys(0, 0);
    url.hash(&mut hasher);
    let hash_value = hasher.finish();
    let encoded_hash = encode(hash_value);
    encoded_hash
}

fn main() {
    let mut url_map = HashMap::new();

    loop {
        println!("Enter a URL or type 'exit' to quit:");

        let mut url = String::new();
        io::stdin().read_line(&mut url).expect("Failed to read line");
        let url = url.trim();

        if url == "exit" {
            break;
        }

        let alias = generate_alias(url);
        url_map.insert(alias.clone(), url.to_string());

        println!("Shortened URL: {}\n", alias);
    }

    println!("URLs and their aliases:");
    for (alias, url) in url_map.iter() {
        println!("{} => {}", alias, url);
    }
}

