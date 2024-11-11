use std::time::Duration;

use ashscript_types::{actions::ActionsByKind, components::tile::Tile, global::Global, keyframe::KeyFrame, map::Map, objects::GameObjectKind, player::PlayerId, world::deserialize_world_data};
use hashbrown::HashMap;
use hecs::{Entity, World};
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

#[derive(Default)]
pub struct GameState {
    pub map: Map,
    pub global: Global,
    pub world: World,
    pub bots: HashMap<PlayerId, Bot>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn despawn_entity(&mut self, entity: Entity) -> Option<()> {

        let (tile, kind) = self.world.query_one_mut::<(&Tile, &GameObjectKind)>(entity).ok()?;

        self.map.remove_entity(&tile.hex, *kind);
        self.world.despawn(entity).ok()
    }
}

#[derive(Default)]
pub struct BotGameState {
    pub map: Map,
    pub global: Global,
    pub world: World,
    pub me: Me,
}

impl BotGameState {
    pub fn from_keyframe(keyframe: KeyFrame) -> Option<Self> {

        let world = deserialize_world_data(keyframe.world_data)?;

        Some(Self {
            map: keyframe.map,
            global: keyframe.global,
            world,
            ..Default::default()
        })
    }
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct Me {
    pub id: PlayerId,
    pub name: String,
}

impl Me {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

// pub type PlayerMemories = HashMap<Uuid, PlayerMemory>;

// #[derive(Default)]
// pub struct PlayerMemory {
//     pub memory: HashMap<String, String>,   
// }

// impl PlayerMemory {
//     pub fn new() -> Self {
//         Self {
//             memory: HashMap::new(),
//         }
//     }

//     /// How much data is in self.memory, in terms of kb(?)
//     pub fn size() -> u32 {
//         0
//     }
// }

#[derive(Default, Clone, Serialize)]
pub struct Bot {
    pub id: PlayerId,
    /// For internal persistent storage
    pub memory: HashMap<String, String>,
}

impl Bot {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            id: player_id,
            ..Default::default()
        }
    }
}

pub type Bots = HashMap<Uuid, Bot>;