use godot::prelude::*;
mod hex;
mod sphere_grid;
mod math;

struct SphericalHexExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SphericalHexExtension {}

