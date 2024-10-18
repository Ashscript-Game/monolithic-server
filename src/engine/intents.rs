use ashscript_types::{
    actions::{self, ActionsByKind}, constants::structures::IMPASSIBLE_GAME_OBJECTS, intents::{self, Intent, Intents}, objects::{Attackable, GameObjectKind}, resource::Resource, structures::factory
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

    actions_by_kind
}

fn create_turret_attack_actions(
    game_state: &mut GameState,
    intents: &[intents::TurretAttack],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let (damage, cost) = {
            let Some(turret) = game_state.map.turret_at(&intent.turret_hex) else {
                continue;
            };

            let cost = turret.attack_cost();
            if turret.energy < cost {
                continue;
            }

            (turret.damage(), cost)
        };

        match intent.target_kind {
            Attackable::Unit => {
                let Some(_) = game_state.map.unit_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
            Attackable::Factory => {
                let Some(_) = game_state.map.factory_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
            Attackable::Turret => {
                let Some(_) = game_state.map.turret_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
        };

        let Some(turret) = game_state.map.turret_at_mut(&intent.turret_hex) else {
            continue;
        };

        turret.future_energy = 0.max(turret.energy - cost);

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
        let (damage, cost) = {
            let Some(unit) = game_state.map.unit_at_mut(&intent.attacker_hex) else {
                continue;
            };

            let cost = unit.attack_cost();

            if unit.energy < cost {
                continue;
            }

            (unit.damage(), cost)
        };

        match intent.target_kind {
            Attackable::Unit => {
                let Some(_) = game_state.map.unit_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
            Attackable::Factory => {
                let Some(_) = game_state.map.factory_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
            Attackable::Turret => {
                let Some(_) = game_state.map.turret_at_mut(&intent.target_hex) else {
                    continue;
                };
            }
        };

        let Some(unit) = game_state.map.unit_at_mut(&intent.attacker_hex) else {
            continue;
        };

        unit.future_energy = 0.max(unit.energy - cost);

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
) -> bool {
    let Some(unit) = game_state.map.unit_at_mut(&from) else {
        return false;
    };

    let cost = unit.weight();
    if cost > unit.energy {
        return false;
    }

    for kind in IMPASSIBLE_GAME_OBJECTS.iter() {
        match kind {
            GameObjectKind::Turret => {
                if game_state.map.turret_at(&to).is_some() {
                    return false;
                }
            }
            GameObjectKind::Factory => {
                if game_state.map.factory_at(&to).is_some() {
                    return false;
                }
            }
            GameObjectKind::Unit => {
                if game_state.map.unit_at(&to).is_some() {
                    if let Some(next_to) = intents_from_to.get(&to) {
                        if !create_unit_move_action(
                            (to, *next_to),
                            intents_from_to,
                            game_state,
                            actions_by_kind,
                        ) {
                            return false;
                        }
                    };
                }
            }
            _ => {
                return false;
            }
        }
    }

    actions_by_kind
        .unit_move
        .push(actions::UnitMove { from, to, cost });

    true
}

fn create_factory_spawn_unit_actions(
    game_state: &mut GameState,
    intents: &[intents::FactorySpawnUnit],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(factory) = game_state.map.factory_at(&intent.factory_hex) else {
            continue;
        };
        println!("cost check {:?}", factory.storage.resources.get(&Resource::Metal));
        let cost = intent.body.cost();
        if !factory.storage.has_sufficient(&cost) {
            continue;
        }
        println!("succeeded cost check");
        let Some(out) = find_unit_out(&intent.out, intent.factory_hex, game_state) else {
            continue;
        };

        let Some(factory) = game_state.map.factory_at_mut(&intent.factory_hex) else {
            continue;
        };

        // should subtract from future_resources
        let Ok(()) = factory.storage.subtract_many_checked(&cost) else {
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
            });
    }
}

fn create_unit_spawn_unit_actions(
    game_state: &mut GameState,
    intents: &[intents::UnitSpawnUnit],
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let Some(unit) = game_state.map.unit_at(&intent.unit_hex) else {
            continue;
        };

        let cost = intent.body.cost();
        if !unit.storage.has_sufficient(&cost) {
            continue;
        };

        let Some(out) = find_unit_out(&intent.out, intent.unit_hex, game_state) else {
            continue;
        };

        let Some(unit) = game_state.map.unit_at_mut(&intent.unit_hex) else {
            continue;
        };

        // should subtract from future_resources
        let Ok(()) = unit.storage.subtract_many_checked(&cost) else {
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
            });
    }
}

// This should be a utility function somewhere
fn find_unit_out(outs: &Option<Vec<Hex>>, from: Hex, game_state: &GameState) -> Option<Hex> {
    if let Some(outs) = outs {
        for out in outs.iter() {
            for kind in IMPASSIBLE_GAME_OBJECTS.iter() {
                let is_impassible = match kind {
                    GameObjectKind::Turret => game_state.map.turret_at(out).is_some(),
                    GameObjectKind::Factory => game_state.map.factory_at(out).is_some(),
                    GameObjectKind::Unit => game_state.map.unit_at(out).is_some(),
                    _ => false,
                };

                if !is_impassible {
                    return Some(*out);
                }
            }
        }

        return None;
    }

    for out in from.all_neighbors() {
        for kind in IMPASSIBLE_GAME_OBJECTS.iter() {
            let is_impassible = match kind {
                GameObjectKind::Turret => game_state.map.turret_at(&out).is_some(),
                GameObjectKind::Factory => game_state.map.factory_at(&out).is_some(),
                GameObjectKind::Unit => game_state.map.unit_at(&out).is_some(),
                _ => false,
            };

            if !is_impassible {
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