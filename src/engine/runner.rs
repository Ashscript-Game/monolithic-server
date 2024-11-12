use crate::{
    engine::{
        client::emit_tick, components::delete_0_health, game_objects::update_resources, process_actions::process_actions, unit::units_generate_energy
    },
    game_state::GameState, simulations,
};
use std::{sync::Arc, time::{self, Duration}};
use ashscript_types::components::health::Health;
use tokio::{sync::broadcast::Sender, time::sleep};

use super::{
    intents::get_and_process_intents,
    unit::{age_units, delete_old_units},
};

pub async fn runner(game_state: &mut GameState, mut sender: Sender<Arc<Vec<u8>>>) {
    loop {
        tick(game_state, &mut sender).await;
    }
}

pub async fn tick(game_state: &mut GameState, sender: &mut Sender<Arc<Vec<u8>>>) {
    println!("\n starting tick: {}", game_state.global.tick);

    let start_time = time::Instant::now();

    let actions_by_kind = get_and_process_intents(game_state);

    emit_tick(game_state, &actions_by_kind, sender);

    process_actions(game_state, &actions_by_kind);
    
    println!("actions_by_kind: {:?}", actions_by_kind);

    age_units(game_state);
    delete_old_units(game_state);
    delete_0_health(game_state);
    units_generate_energy(game_state);

    update_resources(game_state);

    game_state.global.tick += 1;

    simulations::basic::update(game_state);

    sleep(Duration::new(0, 500)).await;

    // record how long the tick took

    game_state.global.last_tick_duration = start_time.elapsed();
}