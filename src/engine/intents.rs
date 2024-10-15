use ashscript_types::{
    actions::{self, ActionsByKind},
    constants::structures::IMPASSIBLE_GAME_OBJECTS,
    intents::{
        self, Intent, Intents,
    },
    objects::{Attackable, GameObjectKind},
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
            &mut intents_from_to,
            game_state,
            actions_by_kind,
        );
    }
}

fn create_unit_move_action(
    (from, to): (Hex, Hex),
    intents_from_to: &mut HashMap<Hex, Hex>,
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
                        )
                        {
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

fn create_factory_spawn_unit_actions() {

}
