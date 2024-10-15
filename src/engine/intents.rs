use ashscript_types::{
    actions::{self, ActionsByKind},
    intents::{
        self, FactorySpawnUnit, Intent, IntentName, Intents, ResourceTransfer, TurretAttack,
        UnitAttack, UnitMove, UnitSpawnUnit,
    },
    objects::Attackable,
};
use enum_map::{enum_map, EnumMap};

use crate::game_state::GameState;

use super::bots::run_bots;

/* pub type IntentsByAction = EnumMap<IntentName, Vec<Intent>>; */
/// Probably makes sense to just have bots return this directly, so that we don't have to construct it from the vec
#[derive(Default)]
pub struct IntentsByKind {
    pub unit_move: Vec<UnitMove>,
    pub unit_attack: Vec<UnitAttack>,
    pub turret_attack: Vec<TurretAttack>,
    pub factory_spawn_unit: Vec<FactorySpawnUnit>,
    pub unit_spawn_unit: Vec<UnitSpawnUnit>,
    pub resource_transfer: Vec<ResourceTransfer>,
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

    actions_by_kind
}

fn create_turret_attack_actions(
    game_state: &mut GameState,
    intents: &Vec<intents::TurretAttack>,
    actions_by_kind: &mut ActionsByKind,
) {
    for intent in intents.iter() {
        let (max_damage, cost) = {
            let Some(turret) = game_state.map.turret_at(&intent.turret_hex) else {
                continue;
            };

            let cost = turret.attack_cost();
            if turret.energy < cost {
                continue;
            }
            
            (turret.damage(), cost)
        };

        let damage = match intent.target_kind {
            Attackable::Unit(..) => {
                let Some(target) = game_state.map.unit_at_mut(&intent.target_hex) else {
                    continue;
                };

                if target.future_health == 0 {
                    continue;
                }

                let damage = max_damage.min(target.future_health);
                target.future_health -= damage;

                damage
            }
            Attackable::Factory(..) => {
                let Some(target) = game_state.map.factory_at_mut(&intent.target_hex) else {
                    continue;
                };

                if target.future_health == 0 {
                    continue;
                }

                let damage = max_damage.min(target.future_health);
                target.future_health -= damage;

                damage
            }
            Attackable::Turret(..) => {
                let Some(target) = game_state.map.turret_at_mut(&intent.target_hex) else {
                    continue;
                };

                if target.future_health == 0 {
                    continue;
                }

                let damage = max_damage.min(target.future_health);
                target.future_health -= damage;

                damage
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
