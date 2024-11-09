use ashscript_types::{components::{owner::Owner, storage::Storage, tile::Tile}, objects::GameObjectKind, player::PlayerId, structures::{factory::Factory, turret::Turret}};
use hecs::Entity;
use hexx::Hex;

use crate::game_state::GameState;

pub fn spawn_turret(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    game_state.world.spawn((Turret::default(), Tile::new(hex), Owner(owner_id)))
}

pub fn spawn_factory(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    game_state.world.spawn((Factory::default(), Tile::new(hex), Owner(owner_id), Storage::default()))
}