use ashscript_types::{
    actions::{self, ActionsByKind},
    intents::Intents,
    keyframe::KeyFrame,
};
use hashbrown::HashMap;
use uuid::Uuid;

use crate::{
    ai::{generalist, shared::BotMemory},
    game_state::{self, Bot, BotGameState, GameState},
};

use super::intents::IntentsByKind;

pub type PlayerScript = fn(&mut BotGameState, &mut BotMemory) -> Intents;
pub type PlayerScripts = HashMap<Uuid, PlayerScript>;

pub fn run_bots(game_state: &mut game_state::GameState) -> IntentsByKind {
    let bot_scripts = get_player_scripts(game_state);

    let mut intents_by_action = IntentsByKind::new();

    for (player_id, script) in bot_scripts {
        let keyframe = KeyFrame::from_existing(
            game_state.map.clone(),
            &game_state.world,
            game_state.global.clone(),
            ActionsByKind::new(),
        );
        let Some(mut bot_game_state) = game_state::BotGameState::from_keyframe(keyframe) else {
            continue;
        };
        
        bot_game_state.me.id = player_id;

        let Some(bot) = game_state.bots.get_mut(&player_id) else {
            continue;
        };

        // Need to convert agnostic bot memory into known bot memory
        let mut new_intents = script(&mut bot_game_state, &mut BotMemory::new() /* bot.memory */);
        intents_by_action.add_intents(&mut new_intents);
    }

    intents_by_action
}

fn get_player_scripts(game_state: &mut GameState) -> PlayerScripts {
    let mut scripts: PlayerScripts = HashMap::new();

    for player_id in game_state.global.players.keys() {
        scripts.insert(*player_id, generalist::main);

        if game_state.bots.get_mut(player_id).is_none() {
            game_state.bots.insert(*player_id, Bot::new(*player_id));
        };
    }

    scripts
}
