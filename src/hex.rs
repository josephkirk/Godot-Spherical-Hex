use godot::prelude::*;
use glam::Vec3;

#[derive(Debug)]
pub struct HexCoord {
    pub q: i32,  // x axis
    pub r: i32,  // y axis
    pub s: i32,  // z axis (derived: s = -q-r)
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        HexCoord {
            q,
            r,
            s: -q - r,
        }
    }

    pub fn neighbors(&self) -> Vec<HexCoord> {
        let directions = [
            (1, 0), (1, -1), (0, -1),
            (-1, 0), (-1, 1), (0, 1)
        ];

        directions.iter()
            .map(|(dq, dr)| HexCoord::new(self.q + dq, self.r + dr))
            .collect()
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct HexTile {
    #[base]
    base: Base<Node3D>,
    coord: HexCoord,
    world_pos: Vec3,
}

#[godot_api]
impl HexTile {
    #[func]
    pub fn get_neighbors(&mut self) -> Vec<Gd<HexTile>> {
        // TODO: Implement neighbor lookup
        Vec::new()
    }

    #[func]
    pub fn get_coordinate(&self) -> Vector3 {
        Vector3::new(self.coord.q as f32, self.coord.r as f32, self.coord.s as f32)
    }
}

#[godot_api]
impl INode3D for HexTile {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            coord: HexCoord::new(0, 0),
            world_pos: Vec3::ZERO,
        }
    }

    fn ready(&mut self) {
        // Initialize tile
    }
}
