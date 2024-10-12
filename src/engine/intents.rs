use ashscript_types::intents::{Intent, IntentName, Intents};
use enum_map::{enum_map, EnumMap};

pub type IntentsByAction = EnumMap<IntentName, Vec<Intent>>;

pub fn organize_intents(intents: &Intents) -> IntentsByAction {
    let by_action: IntentsByAction = enum_map! {
        /* IntentName::UnitAttack => vec![],
        IntentName::UnitMove => vec![],
        IntentName::TurretAttack => vec![],
        IntentName::FactorySpawn => vec![],
        IntentName::ResourceTransfer => vec![], */
        _ => vec![],
    };

    by_action
}