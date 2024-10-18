use ashscript_types::unit::{UnitBody, UnitPart};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STARTING_UNIT_BODY: UnitBody = UnitBody::from_vec(vec![
        (UnitPart::Generate, 15),
        (UnitPart::Fabricate, 3),
        (UnitPart::Convert, 5),
        (UnitPart::Ranged, 6),
        (UnitPart::Shield, 3),
        (UnitPart::RangeImprovement, 4),
        (UnitPart::DamageImprovement, 2),
    ]);
}