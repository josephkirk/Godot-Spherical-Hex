use godot::prelude::*;
use glam::Vec3;
use crate::hex::HexTile;
use crate::math::projection::{generate_icosahedron, project_point_to_sphere, IcosahedronFace};
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SphericalHexGrid {
    #[base]
    base: Base<Node3D>,
    radius: f32,
    resolution: i32,
    faces: Vec<IcosahedronFace>,
    tiles: HashMap<(i32, i32), Gd<HexTile>>,
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

        // Create hex tiles at face centers
        for face in &self.faces {
            let center = project_point_to_sphere(face.center(), self.radius);
            self.create_hex_tile_at(center, face.normal);
        }
    }

    fn create_hex_tile_at(&mut self, position: Vec3, normal: Vec3) {
        let scene = &self.base.get_scene();
        let mut tile: Gd<HexTile> = HexTile::new_alloc();
        
        // Set tile transform
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

        tile.set_basis(rotation);
        tile.set_position(Vector3::new(position.x, position.y, position.z));
        
        // Add tile to grid
        self.base.add_child(tile.share().upcast());
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
            faces: Vec::new(),
            tiles: HashMap::new(),
        }
    }

    fn ready(&mut self) {
        // Generate initial grid
        self.generate_grid();
    }
}
