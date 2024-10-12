use ashscript_types::intents::Intents;
use hashbrown::HashMap;
use uuid::Uuid;

use crate::{ai::{generalist, shared::BotMemory}, game_state::{self, BotGameState, GameState}};

use super::intents::IntentsByAction;

pub type PlayerScript = fn(&BotGameState, &mut BotMemory) -> Intents;
pub type PlayerScripts = HashMap<Uuid, PlayerScript>;

pub fn run_bots(game_state: &mut game_state::GameState) -> IntentsByAction {
    let bot_game_state = game_state::BotGameState::new(game_state);
    let bot_scripts = get_player_scripts(game_state);

    let mut intents_by_action = IntentsByAction::new();

    for (bot_id, script) in bot_scripts {

        let Some(_) = game_state.bots.get_mut(&bot_id) else { continue };

        // Need to convert agnostic bot memory into known bot memory
        let mut new_intents = script(&bot_game_state, &mut BotMemory::new());
        intents_by_action.add_intents(&mut new_intents);
    }

    intents_by_action
}

fn get_player_scripts(game_state: &mut GameState) -> PlayerScripts {
    let mut scripts: PlayerScripts = HashMap::new();

    for bot in game_state.bots.values() {
        scripts.insert(bot.id, generalist::main);
    }

    scripts
}