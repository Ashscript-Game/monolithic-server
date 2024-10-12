use ashscript_types::{intents::{FactorySpawn, ResourceTransfer, TurretAttack, UnitAttack, UnitMove}, intents::{Intent, IntentName, Intents}};
use enum_map::{enum_map, EnumMap};

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

    pub fn add_intent(&mut self, intent: Intent) {
        match intent {
            Intent::UnitMove(unit_move) => self.unit_move.push(unit_move),
            Intent::UnitAttack(unit_attack) => self.unit_attack.push(unit_attack),
            Intent::TurretAttack(turret_attack) => self.turret_attack.push(turret_attack),
            Intent::FactorySpawn(factory_spawn) => self.factory_spawn.push(factory_spawn),
            Intent::ResourceTransfer(resource_transfer) => self.resource_transfer.push(resource_transfer),
        }
    }
}

/// Consumes a list of intents and returns a list of intents by action
pub fn organize_intents(intents: &mut Intents) -> IntentsByAction {
    let mut by_action = IntentsByAction::default();

    while let Some(intent) = intents.pop() {
        by_action.add_intent(intent);
    }
    
    by_action
}