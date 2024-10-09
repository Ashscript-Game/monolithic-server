use std::collections::HashMap;

use ashscript_types::{structures::turret::Turret, unit::Unit};
use hexx::Hex;

pub struct GameState {
    pub units: HashMap<String, Unit>,
    pub turret: HashMap<String, Turret>,
}

impl GameState {
    pub fn age_all(&mut self) {
        for (_unit_id, unit) in self.units.iter_mut() {
            unit.age += 1;
        }
    }

    pub fn move_unit(&mut self, unit: String, dir: u8) {
        if let Some(unit) = self.units.get_mut(&unit) {
            unit.hex = Hex::new(unit.hex.x, unit.hex.y + 1);
        }
    }
}