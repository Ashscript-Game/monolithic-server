use ashscript_types::unit::{UnitBody, UnitPart};
use enum_map::enum_map;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STARTING_UNIT_BODY: UnitBody = UnitBody(enum_map! {
        UnitPart::Generate => 15,
        UnitPart::Fabricate => 3,
        UnitPart::Convert => 5,
        UnitPart::Ranged => 6,
        UnitPart::Shield => 3,
        UnitPart::RangeImprovement => 4,
        UnitPart::DamageImprovement => 2,
        _ => 0,
    });
}