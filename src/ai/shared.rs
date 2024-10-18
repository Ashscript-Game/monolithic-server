
use enum_map::{enum_map, Enum, EnumMap};
use hashbrown::HashSet;
use hexx::Hex;
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

#[derive(Default, Debug)]
pub struct BotState {
    pub unit_hexes_by_role: EnumMap<UnitRole, HashSet<Hex>>,
}

impl BotState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BotMemory {
    pub features: AiFeatures,
    pub units_by_role: EnumMap<UnitRole, Vec<u32>>
}

impl BotMemory {
    pub fn new() -> Self {
        Self {
            features: AiFeatures::new(),
            ..Default::default()
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

#[derive(Enum, Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnitRole {
    Scout,
    Extractor,
    Attacker,
    Defender,
    Builder,   
    Hauler,
    Unknown,
}