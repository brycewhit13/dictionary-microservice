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
pub async fn make_request(word: &str) -> Vec<Dictionary> {
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
pub fn parse_response(response: String) -> Vec<Dictionary> {
    // parse the response
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    let response = response.as_array().unwrap();
    let response = response[0].as_object().unwrap();

    // Track dictionary objects for each definition
    let mut def_dicts_all: Vec<Dictionary> = Vec::new();
    let word_of_interest = response["word"].as_str().unwrap().to_string();

    // Loop through the different definitions
    for def in response["meanings"].as_array().unwrap() {
        // create a new dictionary object
        let mut dictionary = Dictionary {
            word: String::new(),
            definition: String::new(),
            part_of_speech: String::new(),
            synonyms: Vec::new(),
            antonyms: Vec::new(),
        };

        // store the word
        dictionary.word = word_of_interest.clone();

        // get the definition
        dictionary.definition = def["definitions"][0]["definition"]
            .as_str()
            .unwrap()
            .to_string();

        // get the part of speech
        dictionary.part_of_speech = def["partOfSpeech"]
        .as_str()
        .unwrap()
        .to_string();

        // get the synonyms
        let synonyms = def["synonyms"]
            .as_array()
            .unwrap();
        for synonym in synonyms {
            dictionary
                .synonyms
                .push(synonym.as_str().unwrap().to_string());
        }

        // get the antonyms
        let antonyms = def["antonyms"]
            .as_array()
            .unwrap();
        for antonym in antonyms {
            dictionary
                .antonyms
                .push(antonym.as_str().unwrap().to_string());
        }
        
        // add the dictionary to the vector
        def_dicts_all.push(dictionary);
    }

    // return the list of dictionaries
    def_dicts_all
}

// Function to convert a dictionary to a string
pub fn dict_to_string(dictionary: Vec<Dictionary>) -> String {
    let mut final_string = String::new();
    
    // Push the word to the final string
    final_string.push_str(&format!("Word: {}\n\n", dictionary[0].word));

    // Loop through the dictionaries
    for dict in dictionary {
        // push the definition and part of speech to the final string
        final_string.push_str(&format!(
            "Definition ({}): {}\n",
            dict.part_of_speech, dict.definition
        ));

        // push the synonyms and antonyms to the final string
        if dict.synonyms.len() > 0 || dict.antonyms.len() > 0 {
            final_string.push_str(&format!("Synonyms: {:?}\n", dict.synonyms));
            final_string.push_str(&format!("Antonyms: {:?}\n\n", dict.antonyms));
        }
        else{
            final_string.push_str("\n");
        }

        /*
        final_string.push_str(&format!(
            "Word: {}\nDefinition: {}\nPart of Speech: {}\nSynonyms: {:?}\nAntonyms: {:?}\n\n",
            dict.word,
            dict.definition,
            dict.part_of_speech,
            dict.synonyms,
            dict.antonyms
        )); */
    }
    
    // return the final string
    final_string
}
