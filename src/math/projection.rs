use glam::{Vec3, vec3};

// Golden ratio components for icosahedron construction
const PHI: f32 = 1.618033988749895;
const PHI_NORM: f32 = 0.8506508083520399; // 1/sqrt(1 + phi^2)
const ONE_NORM: f32 = 0.5257311121191336; // 1/sqrt(1 + phi^2)

#[derive(Debug)]
pub struct IcosahedronFace {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
}

impl IcosahedronFace {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        let normal = (v2 - v1).cross(v3 - v1).normalize();
        Self {
            vertices: [v1, v2, v3],
            normal,
        }
    }

    pub fn subdivide(&self, depth: i32) -> Vec<IcosahedronFace> {
        if depth == 0 {
            return vec![self.clone()];
        }

        // Get midpoints of each edge
        let mid1 = (self.vertices[0] + self.vertices[1]).normalize();
        let mid2 = (self.vertices[1] + self.vertices[2]).normalize();
        let mid3 = (self.vertices[2] + self.vertices[0]).normalize();

        // Create four new faces
        let f1 = IcosahedronFace::new(self.vertices[0], mid1, mid3);
        let f2 = IcosahedronFace::new(mid1, self.vertices[1], mid2);
        let f3 = IcosahedronFace::new(mid3, mid2, self.vertices[2]);
        let f4 = IcosahedronFace::new(mid1, mid2, mid3);

        // Recursively subdivide each new face
        let mut faces = Vec::new();
        for face in [f1, f2, f3, f4] {
            faces.extend(face.subdivide(depth - 1));
        }

        faces
    }

    pub fn center(&self) -> Vec3 {
        ((self.vertices[0] + self.vertices[1] + self.vertices[2]) / 3.0).normalize()
    }
}

impl Clone for IcosahedronFace {
    fn clone(&self) -> Self {
        Self {
            vertices: self.vertices.clone(),
            normal: self.normal,
        }
    }
}

pub fn generate_icosahedron() -> Vec<IcosahedronFace> {
    // Generate the 12 vertices of the icosahedron
    let vertices = [
        vec3(ONE_NORM, 0.0, PHI_NORM),      // 0
        vec3(-ONE_NORM, 0.0, PHI_NORM),     // 1
        vec3(ONE_NORM, 0.0, -PHI_NORM),     // 2
        vec3(-ONE_NORM, 0.0, -PHI_NORM),    // 3
        vec3(0.0, PHI_NORM, ONE_NORM),      // 4
        vec3(0.0, -PHI_NORM, ONE_NORM),     // 5
        vec3(0.0, PHI_NORM, -ONE_NORM),     // 6
        vec3(0.0, -PHI_NORM, -ONE_NORM),    // 7
        vec3(PHI_NORM, ONE_NORM, 0.0),      // 8
        vec3(-PHI_NORM, ONE_NORM, 0.0),     // 9
        vec3(PHI_NORM, -ONE_NORM, 0.0),     // 10
        vec3(-PHI_NORM, -ONE_NORM, 0.0)     // 11
    ];

    // Define the 20 faces of the icosahedron
    let face_indices = [
        // Top pentagon
        [0, 4, 8],  [0, 8, 10], [0, 10, 5], [0, 5, 1], [0, 1, 4],
        // Middle strip
        [4, 1, 9],  [8, 4, 6],  [10, 8, 2], [5, 10, 7], [1, 5, 11],
        // Bottom pentagon
        [3, 6, 9],  [3, 9, 11], [3, 11, 7], [3, 7, 2], [3, 2, 6],
        // Connecting triangles
        [9, 6, 4],  [6, 2, 8],  [2, 7, 10], [7, 11, 5], [11, 9, 1]
    ];

    face_indices.iter()
        .map(|[i1, i2, i3]| IcosahedronFace::new(
            vertices[*i1], 
            vertices[*i2], 
            vertices[*i3]
        ))
        .collect()
}

pub fn project_point_to_sphere(point: Vec3, radius: f32) -> Vec3 {
    point.normalize() * radius
}

pub fn calculate_face_area(face: &IcosahedronFace) -> f32 {
    let a = face.vertices[1] - face.vertices[0];
    let b = face.vertices[2] - face.vertices[0];
    a.cross(b).length() / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icosahedron_generation() {
        let faces = generate_icosahedron();
        assert_eq!(faces.len(), 20);
        
        // Check that all faces are roughly equilateral
        for face in faces {
            let a = (face.vertices[1] - face.vertices[0]).length();
            let b = (face.vertices[2] - face.vertices[1]).length();
            let c = (face.vertices[0] - face.vertices[2]).length();
            
            assert!((a - b).abs() < 0.0001);
            assert!((b - c).abs() < 0.0001);
        }
    }

    #[test]
    fn test_subdivision() {
        let face = IcosahedronFace::new(
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0)
        );

        let subdivided = face.subdivide(1);
        assert_eq!(subdivided.len(), 4);

        // Test that subdivided faces maintain orientation
        for sub_face in subdivided {
            let normal = sub_face.normal;
            assert!(normal.dot(face.normal) > 0.0);
        }
    }
}
