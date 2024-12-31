use godot::prelude::*;
use glam::Vec3;
use std::collections::HashMap;

use crate::hex::HexTile;
use crate::hex_grid::{HexGrid, HexGridSettings};
use crate::math::projection::{generate_icosahedron, IcosahedronFace};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SphericalHexGrid {
    #[base]
    base: Base<Node3D>,
    radius: f32,
    resolution: i32,
    hex_size: f32,
    faces: Vec<IcosahedronFace>,
    tiles: HashMap<String, Gd<HexTile>>,
}

#[godot_api]
impl SphericalHexGrid {
    #[func]
    pub fn generate_grid(&mut self) {
        // Clear existing tiles
        for (_, tile) in self.tiles.drain() {
            tile.queue_free();
        }

        // Generate base icosahedron and subdivide
        let base_faces = generate_icosahedron();
        self.faces = base_faces.iter()
            .flat_map(|face| face.subdivide(self.resolution))
            .collect();

        // Create hex grid
        let settings = HexGridSettings {
            hex_size: self.hex_size,
            grid_radius: (6.0 * 2.0_f32.powi(self.resolution)) as i32,
        };
        let mut hex_grid = HexGrid::new(settings);

        // Generate hex grid for each face
        for face in &self.faces {
            hex_grid.generate_on_face(face.clone());
        }

        // Create hex tiles for each position
        for (coord, position) in hex_grid.get_all_positions() {
            self.create_hex_tile_at(*position, face_normal_at_point(*position), coord);
        }

        // Set up neighbor relationships
        for (coord, tile) in &self.tiles {
            let neighbor_positions = hex_grid.get_neighbor_positions(coord);
            tile.bind_mutself().connect_with_neighbors(&neighbor_positions);
        }
    }

    fn create_hex_tile_at(&mut self, position: Vec3, normal: Vec3, coord: HexCoord) -> Option<Gd<HexTile>> {
        let scene = &self.base.get_scene();
        let mut tile = HexTile::new_alloc();
        
        // Set tile transform
        let scale = self.hex_size * self.radius;
        let up = Vec3::Y;
        let rotation = if normal.dot(up) > 0.999 {
            Basis::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0)
        } else if normal.dot(up) < -0.999 {
            Basis::from_euler(EulerRot::XYZ, std::f32::consts::PI, 0.0, 0.0)
        } else {
            let right = up.cross(normal).normalize();
            let up = normal.cross(right);
            Basis::from_cols(right, up, normal)
        };

        tile.set_transform(Transform3D::new(
            rotation,
            Vector3::new(position.x, position.y, position.z) * self.radius,
            Vector3::ONE * scale
        ));

        // Store coordinate
        tile.bind_mutself().set_coordinate(coord);
        
        // Add tile to grid
        self.base.add_child(tile.share().upcast());
        self.tiles.insert(coord_to_key(&coord), tile.share());
        
        Some(tile)
    }

    #[func]
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    #[func]
    pub fn set_resolution(&mut self, resolution: i32) {
        self.resolution = resolution.max(0);
    }

    #[func]
    pub fn set_hex_size(&mut self, size: f32) {
        self.hex_size = size;
    }

    #[func]
    pub fn get_tile_at_position(&self, world_pos: Vector3) -> Option<Gd<HexTile>> {
        let pos = Vec3::new(world_pos.x, world_pos.y, world_pos.z);
        let normalized = pos.normalize();
        
        // Find the closest face center
        self.faces.iter()
            .min_by(|a, b| {
                let dist_a = a.center().dot(normalized);
                let dist_b = b.center().dot(normalized);
                dist_b.partial_cmp(&dist_a).unwrap()
            })
            .and_then(|face| {
                self.tiles.values()
                    .min_by(|a, b| {
                        let pos_a = a.get_position();
                        let pos_b = b.get_position();
                        let dist_a = (pos_a - world_pos).length_squared();
                        let dist_b = (pos_b - world_pos).length_squared();
                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                    .map(|tile| tile.share())
            })
    }
}

#[godot_api]
impl INode3D for SphericalHexGrid {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            radius: 1.0,
            resolution: 1,
            hex_size: 0.1,
            faces: Vec::new(),
            tiles: HashMap::new(),
        }
    }

    fn ready(&mut self) {
        // Generate initial grid
        self.generate_grid();
    }
}

fn face_normal_at_point(point: Vec3) -> Vec3 {
    point.normalize()
}

fn coord_to_key(coord: &HexCoord) -> String {
    format!("{}_{}", coord.q, coord.r)
}
