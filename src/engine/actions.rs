use ashscript_types::{
    actions::{self, ActionsByKind},
    components::{
        body::UnitBody,
        energy::Energy,
        health::Health,
        owner::Owner,
        storage::{self, Storage},
        tile::Tile,
    },
    objects::GameObjectKind,
};
use hashbrown::HashMap;
use hexx::Hex;

use crate::game_state::GameState;

use super::generate::component::new_unit;

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
) -> Option<()> {
    let chunk = game_state.map.chunk_at(&from)?;

    let entity = *chunk.entities[GameObjectKind::Unit].get(&from)?;

    if game_state
        .map
        .entity_at(&to, GameObjectKind::Unit)
        .is_some()
    {
        if let Some((next_to, next_cost)) = actions_from_to.get(&to) {
            process_move_action(game_state, actions_from_to, to, *next_to, *next_cost);
        } else {
            return None;
        }
    }

    // The move is considered successful. Move the unit and charge it for doing so

    let unit_energy = game_state.world.query_one_mut::<&mut Energy>(entity).ok()?;
    unit_energy.0 -= cost;

    let chunk = game_state.map.chunk_at_mut(&from)?;
    let _ = chunk.entities[GameObjectKind::Unit].remove(&from)?;

    let new_chunk = game_state.map.chunk_at_mut(&to)?;
    new_chunk.entities[GameObjectKind::Unit].insert(to, entity);

    let tile = game_state.world.query_one_mut::<&mut Tile>(entity).ok()?;
    tile.hex = to;

    Some(())
}

fn process_unit_attack_actions(game_state: &mut GameState, actions: &[actions::UnitAttack]) {
    for action in actions.iter() {
        let Some(attacker_entity) = game_state
            .map
            .entity_at(&action.attacker_hex, GameObjectKind::Unit)
        else {
            continue;
        };
        let Ok(attacker_energy) = game_state
            .world
            .query_one_mut::<&mut Energy>(*attacker_entity)
        else {
            continue;
        };

        attacker_energy.0 = (attacker_energy.0 - action.cost).max(0);

        let Some(target_entity) = game_state
            .map
            .entity_at(&action.target_hex, action.target_kind)
        else {
            continue;
        };
        let Ok(target_health) = game_state
            .world
            .query_one_mut::<&mut Health>(*target_entity)
        else {
            continue;
        };

        target_health.0 = (target_health.0 - action.damage).max(0);
    }
}

fn process_turret_attack_actions(game_state: &mut GameState, actions: &[actions::TurretAttack]) {
    for action in actions.iter() {
        let Some(turret_entity) = game_state
            .map
            .entity_at(&action.turret_hex, GameObjectKind::Unit)
        else {
            continue;
        };
        let Ok(turret_energy) = game_state
            .world
            .query_one_mut::<&mut Energy>(*turret_entity)
        else {
            continue;
        };

        turret_energy.0 -= action.cost;

        let Some(target_entity) = game_state
            .map
            .entity_at(&action.target_hex, action.target_kind)
        else {
            continue;
        };
        let Ok(target_health) = game_state
            .world
            .query_one_mut::<&mut Health>(*target_entity)
        else {
            continue;
        };

        target_health.0 -= action.damage;
    }
}

fn process_factory_spawn_unit_actions(
    game_state: &mut GameState,
    actions: &[actions::FactorySpawnUnit],
) {
    for action in actions.iter() {
        if game_state
            .map
            .entity_at(&action.out, GameObjectKind::Unit)
            .is_some()
        {
            println!("UNIT ALREADY AT HEX TRYING TO SPAWN TO");
            continue;
        };

        let Some(entity) = game_state
            .map
            .entity_at(&action.factory_hex, GameObjectKind::Factory)
        else {
            continue;
        };
        let Ok((storage, owner)) = game_state
            .world
            .query_one_mut::<(&mut Storage, &Owner)>(*entity)
        else {
            continue;
        };

        let Ok(()) = storage.subtract_many_checked(&action.cost) else {
            continue;
        };

        new_unit(
            game_state,
            action.name.clone(),
            action.out,
            action.body,
            action.owner,
        );
    }
}

fn process_resource_transfer_actions(
    game_state: &mut GameState,
    actions: &[actions::ResourceTransfer],
) {
    for action in actions.iter() {
        // Make sure that the sender exists

        let Some(from_entity) = game_state.map.entity_at(&action.from, action.from_kind) else {
            continue;
        };

        // Check if the receiver exists, if so add the resources

        let Some(to_entity) = game_state.map.entity_at(&action.to_hex, action.to_kind) else {
            continue;
        };
        let Ok(to_storage) = game_state.world.query_one_mut::<&mut Storage>(*to_entity) else {
            continue;
        };

        if to_storage
            .add_checked(&action.resource, &action.amount)
            .is_err()
        {
            continue;
        }

        // Remove the sender's resources

        let Ok(from_storage) = game_state.world.query_one_mut::<&mut Storage>(*from_entity) else {
            continue;
        };

        if from_storage
            .subtract_checked(&action.resource, &action.amount)
            .is_err()
        {
            continue;
        }
    }
}
