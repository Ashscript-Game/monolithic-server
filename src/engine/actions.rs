use ashscript_types::{
    actions::{self, ActionsByKind},
    objects::{Attackable, WithStorage},
};
use hashbrown::HashMap;
use hexx::Hex;

use crate::game_state::GameState;

use super::unit::spawn_unit;

pub fn process_actions(game_state: &mut GameState, actions: &ActionsByKind) {
    process_move_actions(game_state, &actions.unit_move);
    process_factory_spawn_unit_actions(game_state, &actions.factory_spawn_unit);
    process_unit_attack_actions(game_state, &actions.unit_attack);
    process_turret_attack_actions(game_state, &actions.turret_attack);
    process_resource_transfer_actions(game_state, &actions.resource_transfer);
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

fn process_unit_attack_actions(game_state: &mut GameState, actions: &[actions::UnitAttack]) {
    for action in actions.iter() {
        let Some(attacker) = game_state.map.unit_at_mut(&action.attacker_hex) else {
            continue;
        };

        attacker.energy -= action.cost;

        match action.target_kind {
            Attackable::Unit => {
                let Some(target) = game_state.map.unit_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
            Attackable::Factory => {
                let Some(target) = game_state.map.factory_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
            Attackable::Turret => {
                let Some(target) = game_state.map.turret_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
        }
    }
}

fn process_turret_attack_actions(game_state: &mut GameState, actions: &[actions::TurretAttack]) {
    for action in actions.iter() {
        let Some(turret) = game_state.map.turret_at_mut(&action.turret_hex) else {
            continue;
        };

        turret.energy -= action.cost;

        match action.target_kind {
            Attackable::Unit => {
                let Some(target) = game_state.map.unit_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
            Attackable::Factory => {
                let Some(target) = game_state.map.factory_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
            Attackable::Turret => {
                let Some(target) = game_state.map.turret_at_mut(&action.target_hex) else {
                    continue;
                };

                target.health -= action.damage;
            }
        }
    }
}

fn process_factory_spawn_unit_actions(
    game_state: &mut GameState,
    actions: &[actions::FactorySpawnUnit],
) {
    for action in actions.iter() {
        let Some(factory) = game_state.map.factory_at_mut(&action.factory_hex) else {
            continue;
        };

        let Ok(()) = factory.storage.subtract_many_checked(&action.cost) else {
            continue;
        };

        if let Some(unit) = game_state.map.unit_at(&action.out) {
            println!("UNIT ALREADY AT HEX TRYING TO SPAWN TO")
        }

        let factory = game_state.map.factory_at(&action.factory_hex).unwrap();

        spawn_unit(
            action.out,
            action.name.clone(),
            action.body,
            factory.owner_id,
            game_state,
        );
    }
}

fn process_resource_transfer_actions(
    game_state: &mut GameState,
    actions: &[actions::ResourceTransfer],
) {
    for action in actions.iter() {

        // Make sure that the sender exists

        match action.from_kind {
            WithStorage::Unit => {
                if game_state.map.unit_at(&action.from).is_none() {
                    continue;
                };
            }
            WithStorage::Factory => {
                if game_state.map.factory_at(&action.from).is_none() {
                    continue;
                };
            }
        }
        
        // Check if the receiver exists, if so add the resources

        match action.to_kind {
            WithStorage::Unit => {
                let Some(object) = game_state.map.unit_at_mut(&action.to_hex) else {
                    continue;
                };

                if object.storage.add_checked(&action.resource, &action.amount).is_err() {
                    continue;
                }
            }
            WithStorage::Factory => {
                let Some(object) = game_state.map.factory_at_mut(&action.to_hex) else {
                    continue;
                };

                if object.storage.add_checked(&action.resource, &action.amount).is_err() {
                    continue;
                }
            }
        }

        // Remove the sender's resources

        match action.from_kind {
            WithStorage::Unit => {
                let Some(object) = game_state.map.unit_at_mut(&action.from) else {
                    continue;
                };

                if object.storage.subtract_checked(&action.resource, &action.amount).is_err() {
                    continue;
                }
            }
            WithStorage::Factory => {
                let Some(object) = game_state.map.factory_at_mut(&action.from) else {
                    continue;
                };

                if object.storage.subtract_checked(&action.resource, &action.amount).is_err() {
                    continue;
                }
            }
        }
    }
}
