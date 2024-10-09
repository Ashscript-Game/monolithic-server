use std::default;

use enum_map::{enum_map, Enum, EnumMap};
use hashbrown::HashSet;
use lazy_static::lazy_static;

#[derive(Default)]
pub struct AiFeatures {
    expand: bool,
    /// A list of structures the bot cannot build
    disabled_structures: HashSet<String>,
    /// A list of technologies the bot cannot use
    disabled_technologies: HashSet<String>,
}

impl AiFeatures {

    pub fn new() -> Self {
        Self {
            expand: true,
            ..Default::default()
        }
    }
}

pub struct Memory {
    features: AiFeatures,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            features: AiFeatures::new(),
        }
    }
}

pub type Stats = EnumMap<Stat, u32>;

#[derive(Enum)]
pub enum Stat {
    EnergyIncome,
    MineralIncome,
    MetalIncome,
    CpuUsage,
}

lazy_static! {
    pub static ref STATS_TO_AVERAGE: EnumMap<Stat, bool> = enum_map! {
        _ => false
    };
}