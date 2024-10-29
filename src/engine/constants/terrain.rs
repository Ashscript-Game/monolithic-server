use lazy_static::lazy_static;
use libnoise::prelude::*;

lazy_static! {
    pub static ref SIMPLEX_GENERATOR: Blend<2, Fbm<2, Simplex<2>>, Scale<2, Worley<2>>, Scale<2, Worley<2>>> = Source::simplex(43)                 // start with simplex noise
    .fbm(5, 0.013, 2.0, 0.5)                        // apply fractal brownian motion
    .blend(                                         // apply blending...
        Source::worley(43).scale([1.25, 1.25]),     // ...with scaled worley noise
        Source::worley(44).scale([0.5, 0.5]));     // ...controlled by other worley noise
}

pub mod resource_noise_tresholds {
    pub const WALL: (f64, f64) = (0.15, 1.);
    pub const COAL: (f64, f64) = (-0.18, -0.18);
    pub const MINERALS: (f64, f64) = (0.148, 0.15);
    pub const SCRAP: (f64, f64) = (-0.23, -0.25);
}