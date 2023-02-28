// Import relevant crates
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dictionary_microservice::{dict_to_string, make_request};

// Create a function for the home page
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// Create a function that runs lib.rs functions to generate response from the API
#[get("/{word}")]
async fn get_dictionary_result(word: web::Path<String>) -> impl Responder {
    // Make the request to the API
    let dictionary = make_request(&word).await;

    // Return the results
    HttpResponse::Ok().body(dict_to_string(dictionary))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");

    // Start the server
    HttpServer::new(|| App::new().service(index).service(get_dictionary_result))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
