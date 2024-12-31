use godot::prelude::*;
use glam::{Vec3, vec3};
use std::collections::HashMap;

use crate::hex::HexTile;
use crate::hex::HexCoord;
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
        let tiles_to_remove: Vec<_> = self.tiles.values().cloned().collect();
        for mut tile in tiles_to_remove {
            tile.queue_free();
        }
        self.tiles.clear();

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
        for (coord, position) in hex_grid.get_all_positions().iter() {
            self.create_hex_tile_at(*position, face_normal_at_point(*position), *coord);
        }

        // First collect all neighbor positions
        let mut neighbor_data = Vec::new();
        for (key, _) in &self.tiles {
            let coord = string_to_coord(key);
            let neighbor_positions = hex_grid.get_neighbor_positions(&coord);
            neighbor_data.push((key.clone(), neighbor_positions));
        }

        // Then update neighbors
        for (key, positions) in neighbor_data {
            let neighbors: Vec<_> = positions.iter()
                .filter_map(|pos| self.get_tile_at_world_pos(*pos))
                .collect();

            if let Some(tile) = self.tiles.get_mut(&key) {
                tile.bind_mut().connect_neighbors(neighbors);
            }
        }
    }

    fn get_tile_at_world_pos(&self, pos: Vec3) -> Option<Gd<HexTile>> {
        let godot_pos = Vector3::new(pos.x, pos.y, pos.z);
        self.tiles.values()
            .min_by(|a, b| {
                let pos_a = a.get_position();
                let pos_b = b.get_position();
                let dist_a = (pos_a - godot_pos).length_squared();
                let dist_b = (pos_b - godot_pos).length_squared();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|tile| tile.clone())
    }

    fn create_hex_tile_at(&mut self, position: Vec3, normal: Vec3, coord: HexCoord) -> Option<Gd<HexTile>> {
        let mut tile = HexTile::new_alloc();
        
        // Set tile transform
        let scale = self.hex_size * self.radius;
        let up = vec3(0.0, 1.0, 0.0);
        
        let basis = if normal.dot(up) > 0.999 {
            Basis::IDENTITY
        } else if normal.dot(up) < -0.999 {
            // Create an inverted basis for bottom-facing tiles
            Basis::from_euler(EulerOrder::XYZ, Vector3::new(std::f32::consts::PI, 0.0, 0.0))
        } else {
            let right = up.cross(normal).normalize();
            let up = normal.cross(right);
            Basis::from_cols(
                Vector3::new(right.x, right.y, right.z),
                Vector3::new(up.x, up.y, up.z),
                Vector3::new(normal.x, normal.y, normal.z)
            )
        };

        let transform = Transform3D::new(
            basis,
            Vector3::new(position.x, position.y, position.z) * self.radius
        );
        
        tile.set_transform(transform);
        tile.bind_mut().set_coordinate(coord.q, coord.r);

        unsafe {
            let parent_node = self.base.to_gd();
            tile.reparent(&parent_node.upcast::<Node>());
        }
        
        let key = coord_to_key(&coord);
        self.tiles.insert(key, tile.clone());
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
            .and_then(|_| self.get_tile_at_world_pos(pos))
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

fn string_to_coord(key: &str) -> HexCoord {
    let parts: Vec<&str> = key.split('_').collect();
    if parts.len() == 2 {
        if let (Ok(q), Ok(r)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            return HexCoord::new(q, r);
        }
    }
    HexCoord::new(0, 0)
}
