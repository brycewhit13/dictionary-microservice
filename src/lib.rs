// Imoprt crates
// TODO: Look into the extraction of everything and how it is printed from the API

// Dictionary object to store data about the word
pub struct Dictionary {
    pub word: String,
    pub definition: String,
    pub part_of_speech: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

// function to make reqwest to free dictionary api
pub async fn make_reqwest(word: &str) -> Dictionary {
    // make request to api
    let response = reqwest::get(format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{word}"
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    // return the parsed response in a dictionary object
    parse_response(response)
}

// function to parse the response
pub fn parse_response(response: String) -> Dictionary {
    // create a new dictionary object
    let mut dictionary = Dictionary {
        word: String::new(),
        definition: String::new(),
        part_of_speech: String::new(),
        synonyms: Vec::new(),
        antonyms: Vec::new(),
    };

    // parse the response
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    let response = response.as_array().unwrap();
    let response = response[0].as_object().unwrap();

    // get the word
    dictionary.word = response["word"].as_str().unwrap().to_string();

    // get the definition
    dictionary.definition = response["meanings"][0]["definitions"][0]["definition"]
        .as_str()
        .unwrap()
        .to_string();

    // get the part of speech
    dictionary.part_of_speech = response["meanings"][0]["partOfSpeech"]
        .as_str()
        .unwrap()
        .to_string();

    // get the synonyms
    let synonyms = response["meanings"][0]["definitions"][0]["synonyms"]
        .as_array()
        .unwrap();
    for synonym in synonyms {
        dictionary
            .synonyms
            .push(synonym.as_str().unwrap().to_string());
    }

    // get the antonyms
    let antonyms = response["meanings"][0]["definitions"][0]["antonyms"]
        .as_array()
        .unwrap();
    for antonym in antonyms {
        dictionary
            .antonyms
            .push(antonym.as_str().unwrap().to_string());
    }

    // return the dictionary object
    dictionary
}

// function to print the contents of the dictionary object
pub fn print_dictionary(dictionary: Dictionary) {
    println!("Word: {}", dictionary.word);
    println!("Definition: {}", dictionary.definition);
    println!("Part of Speech: {}", dictionary.part_of_speech);
    println!("Synonyms: {:?}", dictionary.synonyms);
    println!("Antonyms: {:?}", dictionary.antonyms);
}
