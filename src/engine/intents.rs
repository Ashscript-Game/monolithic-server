use ashscript_types::{
    actions::{self, ActionsByKind},
    components::{
        body::UnitBody, energy::Energy, factory::Factory, storage::Storage, tile::Tile,
        turret::Turret, unit::Unit,
    },
    constants::structures::IMPASSIBLE_GAME_OBJECTS,
    entity,
    intents::{self, Intent, Intents},
    objects::GameObjectKind,
    resource::Resource,
};
use hashbrown::HashMap;
use hexx::Hex;

use crate::game_state::GameState;

use super::bots::run_bots;

/* pub type IntentsByAction = EnumMap<IntentName, Vec<Intent>>; */
/// Probably makes sense to just have bots return this directly, so that we don't have to construct it from the vec
#[derive(Default)]
pub struct IntentsByKind {
    pub unit_move: Vec<intents::UnitMove>,
    pub unit_attack: Vec<intents::UnitAttack>,
    pub turret_attack: Vec<intents::TurretAttack>,
    pub factory_spawn_unit: Vec<intents::FactorySpawnUnit>,
    pub unit_spawn_unit: Vec<intents::UnitSpawnUnit>,
    pub resource_transfer: Vec<intents::ResourceTransfer>,
}

impl IntentsByKind {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Adds the intents from `intents` to `self`, leaving `intents` empty
    pub fn add_intents(&mut self, intents: &mut Intents) {
        while let Some(intent) = intents.pop() {
            self.add_intent(intent);
        }
    }

    fn add_intent(&mut self, intent: Intent) {
        match intent {
            Intent::UnitMove(unit_move) => self.unit_move.push(unit_move),
            Intent::UnitAttack(unit_attack) => self.unit_attack.push(unit_attack),
            Intent::TurretAttack(turret_attack) => self.turret_attack.push(turret_attack),
            Intent::FactorySpawnUnit(factory_spawn) => self.factory_spawn_unit.push(factory_spawn),
            Intent::UnitSpawnUnit(unit_spawn_unit) => self.unit_spawn_unit.push(unit_spawn_unit),
            Intent::ResourceTransfer(resource_transfer) => {
                self.resource_transfer.push(resource_transfer)
            }
        }
    }
}

pub fn get_and_process_intents(game_state: &mut GameState) -> ActionsByKind {
    let intents_by_kind = run_bots(game_state);
    let mut actions_by_kind = ActionsByKind::new();

    create_turret_attack_actions(
        game_state,
        &intents_by_kind.turret_attack,
        &mut actions_by_kind,
    );
    create_unit_attack_actions(
        game_state,
        &intents_by_kind.unit_attack,
        &mut actions_by_kind,
    );
    create_unit_move_actions(game_state, &intents_by_kind.unit_move, &mut actions_by_kind);

    create_factory_spawn_unit_actions(
        game_state,
        &intents_by_kind.factory_spawn_unit,
        &mut actions_by_kind,
    );
    create_unit_spawn_unit_actions(
        game_state,
        &intents_by_kind.unit_spawn_unit,
        &mut actions_by_kind,
    );
    create_resource_transfer_actions(
        game_state,
        &intents_by_kind.resource_transfer,
        &mut actions_by_kind,
    );

    actions_by_kind
}

fn create_turret_attack_actions(
    game_state: &mut GameState,
    intents: &[intents::TurretAttack],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(turret_entity) = game_state
            .map
            .entity_at(&intent.turret_hex, GameObjectKind::Turret)
        else {
            continue;
        };

        let (damage, cost) = {
            let Ok((turret, turret_energy)) = game_state
                .world
                .query_one_mut::<(&Turret, &Energy)>(*turret_entity)
            else {
                continue;
            };

            let cost = turret.attack_cost();
            if turret_energy.0 < cost {
                continue;
            }

            (turret.damage(), cost)
        };

        if game_state
            .map
            .entity_at(&intent.turret_hex, intent.target_kind)
            .is_none()
        {
            continue;
        };

        let Ok(turret_energy) = game_state
            .world
            .query_one_mut::<&mut Energy>(*turret_entity)
        else {
            continue;
        };
        turret_energy.0 = 0.max(turret_energy.0 - cost);

        actions_by_kind.turret_attack.push(actions::TurretAttack {
            turret_hex: intent.turret_hex,
            target_hex: intent.target_hex,
            target_kind: intent.target_kind,
            damage,
            cost,
        });
    }
}

fn create_unit_attack_actions(
    game_state: &mut GameState,
    intents: &[intents::UnitAttack],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(unit_entity) = game_state
            .map
            .entity_at(&intent.attacker_hex, GameObjectKind::Unit)
        else {
            continue;
        };
        let (damage, cost) = {
            let (unit, body, unit_energy) =
                game_state
                    .world
                    .query_one_mut::<(&Unit, &UnitBody, &Energy)>(*unit_entity).ok().unwrap();

            let cost = body.attack_cost();
            if unit_energy.0 < cost {
                continue;
            }

            if intent.attacker_hex.unsigned_distance_to(intent.target_hex) > body.range() {
                continue;
            }

            (body.damage(), cost)
        };

        if game_state
            .map
            .entity_at(&intent.target_hex, intent.target_kind)
            .is_none()
        {
            continue;
        };

        let Ok(unit_energy) = game_state.world.query_one_mut::<&mut Energy>(*unit_entity) else {
            continue;
        };
        unit_energy.0 = unit_energy.0.saturating_sub(cost);

        actions_by_kind.unit_attack.push(actions::UnitAttack {
            attacker_hex: intent.attacker_hex,
            target_hex: intent.target_hex,
            target_kind: intent.target_kind,
            damage,
            cost,
        });
    }
}

fn create_unit_move_actions(
    game_state: &mut GameState,
    intents: &[intents::UnitMove],
    actions_by_kind: &mut ActionsByKind,
) {
    // need a DFS travesal of move intents
    //  for intent in intents
    //    if unit that hasn't moved is at pos
    //      if intent for unit at pos
    //        try to move unit

    // <From, To>
    let mut intents_from_to: HashMap<Hex, Hex> = HashMap::new();
    for intent in intents.iter() {
        intents_from_to.insert(intent.from, intent.to);
    }

    for intent in intents.iter() {
        create_unit_move_action(
            (intent.from, intent.to),
            &intents_from_to,
            game_state,
            actions_by_kind,
        );
    }
}

fn create_unit_move_action(
    (from, to): (Hex, Hex),
    intents_from_to: &HashMap<Hex, Hex>,
    game_state: &mut GameState,
    actions_by_kind: &mut ActionsByKind,
) -> Option<()> {
    let unit_entity = game_state.map.entity_at(&from, GameObjectKind::Unit)?;

    let (unit, body, unit_energy) = game_state
        .world
        .query_one_mut::<(&Unit, &UnitBody, &Energy)>(*unit_entity).ok()?;

    let cost = body.weight() as u32; // round up
    if cost > unit_energy.0 {
        return None;
    }

    for kind in IMPASSIBLE_GAME_OBJECTS.iter() {

        let Some(entity) = game_state.map.entity_at(&to, *kind) else {
            continue;    
        };

        match kind {
            GameObjectKind::Unit => {
                let next_to = intents_from_to.get(&to)?;

                create_unit_move_action((to, *next_to), intents_from_to, game_state, actions_by_kind)?;
            }
            _ => {
                if game_state.map.entity_at(&to, *kind).is_some() {
                    return None;
                }
            }
        }
    }

    actions_by_kind
        .unit_move
        .push(actions::UnitMove { from, to, cost });

    Some(())
}

fn create_factory_spawn_unit_actions(
    game_state: &mut GameState,
    intents: &[intents::FactorySpawnUnit],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(factory_entity) = game_state
            .map
            .entity_at(&intent.factory_hex, GameObjectKind::Factory)
        else {
            continue;
        };

        let Ok((factory, storage)) = game_state
            .world
            .query_one_mut::<(&Factory, &mut Storage)>(*factory_entity)
        else {
            continue;
        };

        let cost = intent.body.cost();
        if !storage.has_sufficient_many(&cost) {
            continue;
        }

        let Some(out) = find_unit_out(&intent.out, intent.factory_hex, game_state) else {
            continue;
        };

        let Ok((factory, storage)) = game_state
            .world
            .query_one_mut::<(&Factory, &mut Storage)>(*factory_entity)
        else {
            continue;
        };

        // should subtract from future_resources
        let Ok(()) = storage.subtract_many_checked(&cost) else {
            continue;
        };

        if game_state
            .map
            .entity_at(&out, GameObjectKind::Unit)
            .is_some()
        {
            println!("UNIT ALREADY AT HEX TRYING TO SPAWN TO");
            continue;
        };

        actions_by_kind
            .factory_spawn_unit
            .push(actions::FactorySpawnUnit {
                factory_hex: intent.factory_hex,
                out,
                body: intent.body,
                name: intent.name.clone(),
                cost,
                owner: intent.owner,
            });
    }
}

fn create_unit_spawn_unit_actions(
    game_state: &mut GameState,
    intents: &[intents::UnitSpawnUnit],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(unit_entity) = game_state
            .map
            .entity_at(&intent.unit_hex, GameObjectKind::Unit)
        else {
            continue;
        };
        let Ok((unit, storage)) = game_state
            .world
            .query_one_mut::<(&Unit, &mut Storage)>(*unit_entity)
        else {
            continue;
        };

        let cost = intent.body.cost();
        if !storage.has_sufficient_many(&cost) {
            continue;
        };

        let Some(out) = find_unit_out(&intent.out, intent.unit_hex, game_state) else {
            continue;
        };

        let Ok((unit, storage)) = game_state
            .world
            .query_one_mut::<(&Unit, &mut Storage)>(*unit_entity)
        else {
            continue;
        };

        // should subtract from future_resources
        let Ok(()) = storage.subtract_many_checked(&cost) else {
            continue;
        };

        actions_by_kind
            .factory_spawn_unit
            .push(actions::FactorySpawnUnit {
                factory_hex: intent.unit_hex,
                out,
                body: intent.body,
                name: intent.name.clone(),
                cost,
                owner: intent.owner,
            });
    }
}

// This should be a utility function somewhere
// Doesn't seem to account for units that have created an action to move to the hex
fn find_unit_out(outs: &Option<Vec<Hex>>, from: Hex, game_state: &GameState) -> Option<Hex> {
    if let Some(outs) = outs {
        for out in outs.iter() {
            for kind in IMPASSIBLE_GAME_OBJECTS.iter() {
                if game_state.map.entity_at(out, *kind).is_none() {
                    return Some(*out);
                }
            }
        }

        return None;
    }

    for out in from.all_neighbors() {
        for kind in IMPASSIBLE_GAME_OBJECTS.iter() {
            if game_state.map.entity_at(&out, *kind).is_none() {
                return Some(out);
            }
        }
    }

    None
}

fn create_resource_transfer_actions(
    game_state: &mut GameState,
    intents: &[intents::ResourceTransfer],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        // Check if the sender has sufficient resources to send

        let Some(from_entity) = game_state.map.entity_at(&intent.from_hex, intent.from_kind) else {
            continue;
        };
        let Ok(from_storage) = game_state.world.query_one_mut::<&Storage>(*from_entity) else {
            continue;
        };

        if !from_storage.has_sufficient(&intent.resource, &intent.amount) {
            continue;
        };

        // Check if the receiver has sufficient capacity and is allowed to receive the resource

        let Some(to_entity) = game_state.map.entity_at(&intent.from_hex, intent.from_kind) else {
            continue;
        };
        let Ok(to_storage) = game_state.world.query_one_mut::<&Storage>(*to_entity) else {
            continue;
        };

        if to_storage.is_allowed(&intent.resource) {
            continue;
        }
        if to_storage.capacity < intent.amount {
            continue;
        }

        // The transfer is valid

        let Ok(from_storage) = game_state.world.query_one_mut::<&mut Storage>(*from_entity) else {
            continue;
        };

        if from_storage
            .subtract_checked(&intent.resource, &intent.amount)
            .is_err()
        {
            continue;
        };

        // validation after discussion component system

        actions_by_kind
            .resource_transfer
            .push(actions::ResourceTransfer {
                resource: intent.resource,
                from_kind: intent.from_kind,
                to_kind: intent.to_kind,
                from: intent.from_hex,
                to_hex: intent.to_hex,
                amount: intent.amount,
            });
    }
}
