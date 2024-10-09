use std::collections::HashMap;

use ashscript_types::{map::Map, structures::turret::Turret};
use hexx::Hex;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct GameState {
    pub map: Map,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}