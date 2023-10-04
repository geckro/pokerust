use rand::Rng;
use std::fs::read_to_string;
use toml;

pub fn get_random_pokemon(num_times: u8) {
    // Reads the TOML file and puts it into var contents
    eprintln!("INFO::read_pokemon_toml_start");
    let contents = match read_to_string("data/pokemon.toml") {
        Ok(contents) => {
            eprintln!("INFO::read_pokemon_toml_end");
            contents
        }
        Err(err) => {
            eprintln!("ERROR::read_toml_pokemon::{}", err);
            return;
        }
    };

    // Parse the TOML data into a JSON value(?)
    eprintln!("INFO::parse_pokemon_toml_start");
    let parsed_toml: toml::Value = match contents.parse() {
        Ok(parsed_toml) => {
            eprintln!("INFO::parsed_pokemon_toml_end");
            parsed_toml
        }
        Err(err) => {
            eprintln!("Error parsing TOML: {}", err);
            return;
        }
    };
    println!("INFO::parsed_pokemon_toml_list:\n{}", parsed_toml);

    // Access the "pokemon" table from the TOML data
    if let Some(pokemon_table) = parsed_toml.get("pokemon") {
        // Check if it's a table
        if let Some(pokemon_table) = pokemon_table.as_table() {
            // Extract Pokémon names (keys in the table)
            let pokemon_names: Vec<&str> = pokemon_table.keys().map(|k| k.as_str()).collect();

            // Check if there are Pokémon names
            if !pokemon_names.is_empty() {
                // Generate a random index to select a random Pokémon
                let mut rng = rand::thread_rng();
                for _ in 0..num_times {
                    let random_index = rng.gen_range(0..pokemon_names.len());
                    let random_pokemon_name = pokemon_names[random_index];

                    if let Some(random_pokemon) = pokemon_table.get(random_pokemon_name) {
                        println!("{} - {:?}", random_pokemon_name, random_pokemon);
                    } else {
                        eprintln!("ERROR::failed_to_get_poke_data");
                    }
                }
            } else {
                eprintln!("ERROR::no_pokemon_found");
            }
        } else {
            eprintln!("ERROR:pokemon_not_a_table");
        }
    } else {
        eprintln!("ERROR:cannot_find_table");
    }
}
