use ashscript_types::objects::{Attackable, WithEnergy, WithStorage};
use enum_iterator::all;

use crate::game_state::GameState;

// The following update functinos are likely only temporary measures while action processing is part of the engine

pub fn update_resources(game_state: &mut GameState) {
    for kind in all::<WithStorage>() {
        for (_, chunk) in game_state.map.chunks.iter_mut() {
            match kind {
                WithStorage::Unit => {
                    for (_, unit) in chunk.units.iter_mut() {
                        unit.storage.future_resources = unit.storage.resources.clone();
                    }
                }
                WithStorage::Factory => {
                    for (_, factory) in chunk.factories.iter_mut() {
                        factory.storage.future_resources = factory.storage.resources.clone();
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_health(game_state: &mut GameState) {
    for kind in all::<Attackable>() {
        for (_, chunk) in game_state.map.chunks.iter_mut() {
            match kind {
                Attackable::Unit => {
                    for (_, unit) in chunk.units.iter_mut() {
                        unit.future_health = unit.health;
                    }
                }
                Attackable::Turret => {
                    for (_, turret) in chunk.turrets.iter_mut() {
                        turret.future_health = turret.health;
                    }
                }
                Attackable::Factory => {
                    for (_, factory) in chunk.factories.iter_mut() {
                        factory.future_health = factory.health;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_energy(game_state: &mut GameState) {
    for kind in all::<WithEnergy>() {
        for (_, chunk) in game_state.map.chunks.iter_mut() {
            match kind {
                WithEnergy::Unit => {
                    for (_, unit) in chunk.units.iter_mut() {
                        unit.future_energy = unit.energy;
                    }
                }
                WithEnergy::Turret => {
                    for (_, turret) in chunk.turrets.iter_mut() {
                        turret.future_energy = turret.energy;
                    }
                }
                WithEnergy::Factory => {
                    for (_, factory) in chunk.factories.iter_mut() {
                        factory.future_energy = factory.energy;
                    }
                }
            }
        }
    }
}
