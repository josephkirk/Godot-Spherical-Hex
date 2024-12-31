use godot::prelude::*;

pub mod hex;
pub mod sphere_grid;
pub mod math;
pub mod hex_grid;

struct SphericalHexExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SphericalHexExtension {}
