use ashscript_types::{global::Global, map::Map, player::PlayerId};
use hashbrown::HashMap;
use hecs::World;
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
}

#[derive(Default, Serialize, Clone)]
pub struct BotGameState {
    pub map: Map,
    pub global: Global,
    pub me: Me,
}

impl BotGameState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            map: /* Map::new(), */game_state.map.clone(),
            global: game_state.global.clone(),
            ..Default::default()
        }
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