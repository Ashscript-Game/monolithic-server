use ashscript_types::actions::{self, ActionsByKind};
use hashbrown::HashMap;
use hexx::Hex;

use crate::game_state::GameState;

pub fn process_actions(game_state: &mut GameState, actions: &ActionsByKind) {
    process_move_actions(game_state, &actions.unit_move);
}

fn process_move_actions(game_state: &mut GameState, actions: &[actions::UnitMove]) {
    let mut actions_from_to: HashMap<Hex, (Hex, u32)> = HashMap::new();
    for action in actions.iter() {
        actions_from_to.insert(action.from, (action.to, action.cost));
    }

    for action in actions.iter() {
        process_move_action(
            game_state,
            &actions_from_to,
            action.from,
            action.to,
            action.cost,
        );
    }
}

fn process_move_action(
    game_state: &mut GameState,
    actions_from_to: &HashMap<Hex, (Hex, u32)>,
    from: Hex,
    to: Hex,
    cost: u32,
) {
    let Some(chunk) = game_state.map.chunks.get_mut(&from) else {
        return;
    };

    let Some(mut unit) = chunk.units.remove(&from) else {
        return;
    };

    if game_state.map.unit_at_mut(&to).is_some() {
        if let Some((next_to, next_cost)) = actions_from_to.get(&to) {
            process_move_action(game_state, actions_from_to, from, *next_to, *next_cost);
        };
    }

    // The move is considered successful. Move the unit and charge it for doing so

    let Some(new_chunk) = game_state.map.chunks.get_mut(&to) else {
        return;
    };

    unit.energy -= cost;
    new_chunk.units.insert(to, unit);
}
