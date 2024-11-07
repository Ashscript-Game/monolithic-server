use crate::{
    engine::{
        actions::process_actions,
        client::emit_tick,
        game_objects::{update_resources},
        unit::units_generate_energy,
    },
    game_state::GameState,
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast::Sender, time::sleep};

use super::{
    intents::get_and_process_intents,
    unit::{age_units, delete_dead_units},
};

pub async fn runner(game_state: &mut GameState, mut sender: Sender<Arc<Vec<u8>>>) {
    loop {
        tick(game_state, &mut sender).await;
    }
}

pub async fn tick(game_state: &mut GameState, sender: &mut Sender<Arc<Vec<u8>>>) {
    println!("\n starting tick: {}", game_state.global.tick);

    let actions_by_kind = get_and_process_intents(game_state);

    process_actions(game_state, &actions_by_kind);

    println!("actions_by_kind: {:?}", actions_by_kind);

    age_units(game_state);
    delete_dead_units(game_state);
    units_generate_energy(game_state);

    update_resources(game_state);

    game_state.global.tick += 1;

    emit_tick(game_state, sender);

    sleep(Duration::from_secs(5)).await;
}