use std::collections::HashMap;
use std::fs::read_to_string;
use toml::value::Value;

pub fn battle_simulation(pokemon: Vec<String>) {
    // Read the types.toml file
    let types_toml = match read_to_string("data/types.toml") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("ERROR::read_types_toml::{}", err);
            return;
        }
    };

    let parsed_toml: toml::Value = match types_toml.parse() {
        Ok(parsed_toml) => parsed_toml,
        Err(err) => {
            eprintln!("ERROR::parse_types_toml::{}", err);
            return;
        }
    };

    // Assume the first two Pokémon in the input vector are battling
    if pokemon.len() >= 2 {
        let pokemon1 = &pokemon[0];
        let pokemon2 = &pokemon[1];

        // Determine the type effectiveness for each Pokémon
        let effectiveness_pokemon1 =
            determine_type_effectiveness(&[pokemon1.as_str()], &parsed_toml);
        let effectiveness_pokemon2 =
            determine_type_effectiveness(&[pokemon2.as_str()], &parsed_toml);

        // Print the battle results
        println!(
            "Battle simulation between {} and {}:\n\n\n",
            pokemon1, pokemon2
        );
        println!(
            "{} is {:?} AGAINST\n {}: {:?}",
            pokemon1, effectiveness_pokemon1, pokemon2, effectiveness_pokemon1
        );
        println!(
            "{} is {:?} AGAINST\n {}: {:?}",
            pokemon2, effectiveness_pokemon2, pokemon1, effectiveness_pokemon2
        );
    } else {
        println!("At least two Pokémon are required for a battle simulation.");
    }
}

fn determine_type_effectiveness(pokemon_types: &[&str], types: &Value) -> HashMap<String, f32> {
    let mut effectiveness = HashMap::new();
    println!("{:?}", types["types"]["fighting"]);
    // Check effectiveness against other types
    for (type_name, other_type) in &[
        ("normal", &types["types"]["normal"]),
        ("fire", &types["types"]["fire"]),
        ("water", &types["types"]["water"]),
        ("electric", &types["types"]["electric"]),
        ("grass", &types["types"]["grass"]),
        ("ice", &types["types"]["ice"]),
        ("fighting", &types["types"]["fighting"]),
        ("poison", &types["types"]["poison"]),
        ("ground", &types["types"]["ground"]),
        ("flying", &types["types"]["flying"]),
        ("psychic", &types["types"]["psychic"]),
        ("bug", &types["types"]["bug"]),
        ("rock", &types["types"]["rock"]),
        ("ghost", &types["types"]["ghost"]),
        ("dragon", &types["types"]["dragon"]),
        ("dark", &types["types"]["dark"]),
        ("steel", &types["types"]["steel"]),
        ("fairy", &types["types"]["fairy"]),
    ] {
        let type_name = type_name.to_string();

        if let Some(weaknesses) = other_type.get("weak_against") {
            let weaknesses = weaknesses
                .as_array()
                .unwrap_or_else(|| panic!("Invalid 'weak_against' format for type: {}", type_name));

            for pokemon_type in pokemon_types {
                if weaknesses.iter().any(|w| w.as_str() == Some(pokemon_type)) {
                    effectiveness.insert(type_name.clone(), 0.5); // Weak against
                }
            }
        }

        if let Some(strengths) = other_type.get("strong_against") {
            let strengths = strengths.as_array().unwrap_or_else(|| {
                panic!("Invalid 'strong_against' format for type: {}", type_name)
            });

            for pokemon_type in pokemon_types {
                if strengths.iter().any(|s| s.as_str() == Some(pokemon_type)) {
                    let entry = effectiveness.entry(type_name.clone()).or_insert(1.0);
                    *entry *= 2.0; // Strong against
                }
            }
        }

        if let Some(no_damage_type) = other_type.get("no_dmg_against") {
            for pokemon_type in pokemon_types {
                if no_damage_type.as_str() == Some(pokemon_type) {
                    effectiveness.insert(type_name.clone(), 0.0); // No damage against
                }
            }
        }
    }

    effectiveness
}
