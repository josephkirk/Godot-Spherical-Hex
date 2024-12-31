use glam::{Vec2, Vec3, Mat3};
use crate::hex::HexCoord;
use crate::math::projection::IcosahedronFace;
use std::collections::HashMap;

pub const SQRT_3: f32 = 1.7320508075688772;

pub struct HexGridSettings {
    pub hex_size: f32,
    pub grid_radius: i32,
}

pub struct HexGrid {
    settings: HexGridSettings,
    hex_positions: HashMap<HexCoord, Vec3>,
    face_grids: Vec<FaceGrid>,
}

struct FaceGrid {
    face: IcosahedronFace,
    local_coords: Vec<HexCoord>,
    center: Vec3,
    orientation: Mat3,
}

impl HexGrid {
    pub fn new(settings: HexGridSettings) -> Self {
        Self {
            settings,
            hex_positions: HashMap::new(),
            face_grids: Vec::new(),
        }
    }

    pub fn generate_on_face(&mut self, face: IcosahedronFace) {
        let center = face.center();
        
        // Calculate face orientation
        let normal = face.normal;
        let tangent = (face.vertices[1] - face.vertices[0]).normalize();
        let bitangent = normal.cross(tangent);
        let orientation = Mat3::from_cols(tangent, bitangent, normal);

        let mut local_coords = Vec::new();
        let radius = self.settings.grid_radius as f32;

        // Generate hex grid in 2D face space
        for q in -self.settings.grid_radius..=self.settings.grid_radius {
            for r in -self.settings.grid_radius..=self.settings.grid_radius {
                let s = -q - r;
                if s.abs() <= self.settings.grid_radius {
                    let coord = HexCoord::new(q, r);
                    let pos_2d = hex_to_pixel(coord, self.settings.hex_size);
                    
                    // Project onto face plane
                    let pos_3d = orientation.mul_vec3(Vec3::new(pos_2d.x, pos_2d.y, 0.0));
                    let world_pos = project_onto_sphere(center + pos_3d);
                    
                    self.hex_positions.insert(coord, world_pos);
                    local_coords.push(coord);
                }
            }
        }

        self.face_grids.push(FaceGrid {
            face,
            local_coords,
            center,
            orientation,
        });
    }

    pub fn get_neighbor_positions(&self, coord: &HexCoord) -> Vec<Vec3> {
        coord.neighbors().iter()
            .filter_map(|n| self.hex_positions.get(n))
            .copied()
            .collect()
    }

    pub fn get_all_positions(&self) -> &HashMap<HexCoord, Vec3> {
        &self.hex_positions
    }

    pub fn find_hex_at_point(&self, point: Vec3) -> Option<HexCoord> {
        // Find closest face first
        let closest_face = self.face_grids.iter()
            .min_by(|a, b| {
                let dist_a = a.center.distance(point);
                let dist_b = b.center.distance(point);
                dist_a.partial_cmp(&dist_b).unwrap()
            })?;

        // Transform point to face local space
        let local_point = closest_face.orientation.transpose() * (point - closest_face.center);
        let hex_coord = pixel_to_hex(Vec2::new(local_point.x, local_point.y), self.settings.hex_size);

        // Verify the hex exists
        if self.hex_positions.contains_key(&hex_coord) {
            Some(hex_coord)
        } else {
            None
        }
    }
}

fn hex_to_pixel(hex: HexCoord, size: f32) -> Vec2 {
    Vec2::new(
        size * (3.0/2.0 * hex.q as f32),
        size * (SQRT_3/2.0 * hex.q as f32 + SQRT_3 * hex.r as f32)
    )
}

fn pixel_to_hex(point: Vec2, size: f32) -> HexCoord {
    let q = (2.0/3.0 * point.x) / size;
    let r = (-1.0/3.0 * point.x + SQRT_3/3.0 * point.y) / size;
    let s = -q - r;

    // Round to nearest hex
    let mut rq = q.round();
    let mut rr = r.round();
    let mut rs = s.round();

    let q_diff = (rq - q).abs();
    let r_diff = (rr - r).abs();
    let s_diff = (rs - s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        rq = -rr - rs;
    } else if r_diff > s_diff {
        rr = -rq - rs;
    } else {
        rs = -rq - rr;
    }

    HexCoord::new(rq as i32, rr as i32)
}

fn project_onto_sphere(point: Vec3) -> Vec3 {
    point.normalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_pixel_to_hex() {
        let coord = HexCoord::new(2, -1);
        let size = 1.0;
        let pixel = hex_to_pixel(coord, size);
        let result = pixel_to_hex(pixel, size);
        assert_eq!(coord.q, result.q);
        assert_eq!(coord.r, result.r);
    }

    #[test]
    fn test_neighbor_positions() {
        let settings = HexGridSettings {
            hex_size: 1.0,
            grid_radius: 2,
        };
        let mut grid = HexGrid::new(settings);
        let face = IcosahedronFace::new(
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        grid.generate_on_face(face);

        let center = HexCoord::new(0, 0);
        let neighbors = grid.get_neighbor_positions(&center);
        assert_eq!(neighbors.len(), 6);
    }
}
