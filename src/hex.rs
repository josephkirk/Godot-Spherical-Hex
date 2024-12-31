use godot::prelude::*;
use glam::Vec3;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct HexCoord {
    pub q: i32,  // x axis
    pub r: i32,  // y axis
    pub s: i32,  // z axis (derived: s = -q-r)
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct HexTile {
    #[base]
    base: Base<Node3D>,
    coord: HexCoord,
    world_pos: Vec3,
    neighbors: Vec<Gd<HexTile>>,
}

#[godot_api]
impl HexTile {
    #[func]
    pub fn get_neighbors(&mut self) -> Vec<Gd<HexTile>> {
        self.neighbors.clone()
    }

    #[func]
    pub fn get_coordinate(&self) -> Vector3 {
        Vector3::new(self.coord.q as f32, self.coord.r as f32, self.coord.s as f32)
    }

    #[func]
    pub fn set_coordinate(&mut self, q: i32, r: i32) {
        self.coord = HexCoord::new(q, r);
    }

    pub fn connect_neighbors(&mut self, neighbors: Vec<Gd<HexTile>>) {
        self.neighbors = neighbors;
    }
}

#[godot_api]
impl INode3D for HexTile {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            coord: HexCoord::new(0, 0),
            world_pos: Vec3::ZERO,
            neighbors: Vec::new(),
        }
    }

    fn ready(&mut self) {
        // Create basic visual representation
        let mut child = Node3D::new_alloc();
        let transform = Transform3D::new(
            Basis::from_diagonal(0.5, 0.2, 0.5),
            Vector3::ZERO
        );
        child.set_transform(transform);
        
        unsafe {
            let parent_node = self.base.to_gd();
            child.reparent(&parent_node.upcast::<Node>());
        }
    }
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
