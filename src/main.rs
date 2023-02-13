// Import relevant crates
// use actix;
use dictionary_microservice::{make_reqwest, print_dictionary};

// Hello World in Rust
#[tokio::main]
async fn main() {
    // Make the request to the API
    let word: &str = "Explore";
    let dictionary = make_reqwest(word).await;

    // Print the results
    print_dictionary(dictionary);
}
