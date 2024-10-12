use ashscript_types::{intents::{FactorySpawn, ResourceTransfer, TurretAttack, UnitAttack, UnitMove}, intents::{Intent, IntentName, Intents}};
use enum_map::{enum_map, EnumMap};

use crate::game_state::GameState;

use super::bots::run_bots;

/* pub type IntentsByAction = EnumMap<IntentName, Vec<Intent>>; */

#[derive(Default)]
pub struct IntentsByAction {
    pub unit_move: Vec<UnitMove>,
    pub unit_attack: Vec<UnitAttack>,
    pub turret_attack: Vec<TurretAttack>,
    pub factory_spawn: Vec<FactorySpawn>,
    pub resource_transfer: Vec<ResourceTransfer>,
}

impl IntentsByAction {
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
            Intent::FactorySpawn(factory_spawn) => self.factory_spawn.push(factory_spawn),
            Intent::ResourceTransfer(resource_transfer) => self.resource_transfer.push(resource_transfer),
        }
    }
}

pub fn get_and_process_intents(game_state: &mut GameState) {
    let intents_by_action = run_bots(game_state);

    
}