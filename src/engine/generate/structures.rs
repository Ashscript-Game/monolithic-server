use ashscript_types::{objects::GameObjectKind, player::PlayerId, structures::{factory::Factory, turret::Turret}};
use hexx::Hex;

use crate::game_state::GameState;

pub fn spawn_structure(game_state: &mut GameState, hex: Hex, owner_id: PlayerId, kind: GameObjectKind) {
    
    let Some(chunk) = game_state.map.chunk_at_mut(&hex) else {
        return;
    };

    match kind {
        GameObjectKind::Turret => {
            chunk.turrets.insert(hex, spawn_turret(hex, owner_id));
        }
        GameObjectKind::Factory => {
            chunk.factories.insert(hex, spawn_factory(hex, owner_id));
        }
        _ => {

        }
    };
}

pub fn spawn_turret(hex: Hex, owner_id: PlayerId) -> Turret {
    Turret {
        hex,
        owner_id,
        ..Default::default()
    }
}

pub fn spawn_factory(hex: Hex, owner_id: PlayerId) -> Factory {
    Factory {
        hex,
        owner_id,
        ..Default::default()
    }
}