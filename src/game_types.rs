use std::default;

use hashbrown::HashMap;
use hexx::Hex;


pub type Chunks = HashMap<ChunkId, Chunk>;

pub type ChunkId = u32;

#[derive(Default)]
pub struct Chunk {
    pub id: ChunkId,
    pub units: HashMap<UnitId, Unit>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub type UnitsByChunk = HashMap<UnitId, Chunk>;

#[derive(Default)]
pub struct Unit {
    pub id: UnitId,
    pub health: u32,
    pub hex: Hex,
}

impl Unit {
    pub fn new(hex: Hex) -> Self {
        Self {
            health: 100,
            hex,
            ..Default::default()
        }
    }
}

pub type UnitId = u32;
pub type ObjectId = u32;

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Resource {
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
}

pub mod actions {
    use hexx::Hex;

    use super::{Resource, UnitId};

    pub struct MoveAction {
        pub unit_id: UnitId,
        pub from: Hex,
        pub to: Hex,
    }

    pub struct AttackAction {
        pub unit_id: UnitId,
        pub target_id: UnitId,
        pub from: Hex,
        pub damage: u32,
    }

    pub struct FactorySpawn {
        pub factory_id: u32,
        pub unit_id: UnitId,
        pub out: Hex,
    }

    pub struct TurretAttack {
        pub turret_id: u32,
        pub target_id: UnitId,
        pub damge: u32,
    }

    pub struct ResourceAttack {
        pub resource: Resource,
        pub amount: u32,
        pub from_id: u32,
        pub to_id: u32,
    }
}