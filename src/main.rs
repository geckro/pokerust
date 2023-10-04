mod battle;
mod pokemon;

fn main() {
    let num_times_to_print = 2;
    let results = pokemon::get_random_pokemon(num_times_to_print);
    let _ = battle::battle_simulation(results);
}
