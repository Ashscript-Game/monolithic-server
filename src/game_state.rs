use ashscript_types::{map::Map, structures::turret::Turret};
use hashbrown::HashMap;
use hexx::Hex;
use serde::Serialize;
use uuid::Uuid;

#[derive(Default)]
pub struct GameState {
    pub map: Map,
    pub bots: Bots,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_bot(&mut self, name: &str) {

        let bot = Bot::new(name);
        self.bots.insert(bot.id, bot);
    }
}

#[derive(Default, Serialize)]
pub struct BotGameState {
    pub map: Map,
}

impl BotGameState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            map: /* Map::new(), */game_state.map.clone(),
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

#[derive(Default)]
pub struct Bot {
    pub id: Uuid,
    pub name: String,
    pub level: u32,
    /// For internal persistent storage
    pub memory: HashMap<String, String>,
    /// For automated ally communication
    pub public_memory: HashMap<String, String>,
}

impl Bot {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

pub type Bots = HashMap<Uuid, Bot>;