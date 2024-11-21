use std::fmt::format;

use ashscript_types::{
    actions::{ActionsByKind, SubstationCollect},
    components::{
        energy::Energy,
        owner::Owner,
        solar_panel::{self, SolarPanel},
        substation::Substation,
        tile::Tile,
        turbine::Turbine,
    },
    objects::GameObjectKind,
};

use crate::game_state::GameState;

pub fn server_actions(game_state: &mut GameState, actions: &mut ActionsByKind) {
    collect_energy(game_state, actions);
}

pub fn collect_energy(game_state: &mut GameState, actions: &mut ActionsByKind) {
    for (entity, (substation, energy, tile, owner)) in
        &mut game_state
            .world
            .query::<(&Substation, &mut Energy, &Tile, &Owner)>()
    {
        let mut energy: u32 = 0;
        let chunks = game_state.map.chunks_in_area(tile.hex, substation.range());

        for chunk in chunks {
            for turbine_entity in chunk.entities[GameObjectKind::Turbine].values() {
                let mut query = game_state
                    .world
                    .query_one::<(&Turbine, &Owner)>(*turbine_entity)
                    .ok()
                    .unwrap();
                let (turbine, turbine_owner) = query.get().unwrap();
                
                if owner.0 == turbine_owner.0 {
                    energy += turbine.output_chunk(chunk);
                    // energy.current = energy.current.saturating_add(turbine.output_chunk(chunk)).min(energy.capacity);
                }
            }

            if game_state.global.is_night() {
                continue;
            }

            for solar_panel_entity in chunk.entities[GameObjectKind::SolarPanel].values() {
                let mut query = game_state
                    .world
                    .query_one::<(&SolarPanel, &Owner)>(*solar_panel_entity)
                    .ok()
                    .unwrap();
                let (solar_panel, solar_panel_owner) = query.get().unwrap();

                if owner.0 == solar_panel_owner.0 {
                    energy += solar_panel.output_chunk(chunk);
                    // energy.current = energy.current.saturating_add(solar_panel.output_chunk(chunk)).min(energy.capacity);
                }
            }
        }

        actions.substation_collect.push(SubstationCollect {
            substation_hex: tile.hex,
            energy_collected: energy,
        });
    }
}
