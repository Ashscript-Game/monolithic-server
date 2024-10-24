use std::time::Duration;

use tokio::time::sleep;

use crate::{engine::{actions::process_actions, game_objects::{update_energy, update_health, update_resources}, unit::units_generate_energy}, game_state::GameState};

use super::{intents::get_and_process_intents, unit::{age_units, delete_dead_units}};

pub async fn runner(game_state: &mut GameState) {

    loop {
        tick(game_state).await;
    }
}

pub async fn tick(game_state: &mut GameState) {
    println!("\n starting tick: {}", game_state.global.tick);

    let actions_by_kind = get_and_process_intents(game_state);

    process_actions(game_state, &actions_by_kind);

    println!("actions_by_kind: {:?}", actions_by_kind);

    age_units(game_state);
    delete_dead_units(game_state);
    units_generate_energy(game_state);

    update_resources(game_state);
    update_energy(game_state);
    update_health(game_state);

    game_state.global.tick += 1;

    sleep(Duration::from_secs(1)).await;
}