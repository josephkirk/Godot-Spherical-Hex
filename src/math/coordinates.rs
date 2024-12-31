use glam::{Vec3, Vec2};

pub fn cartesian_to_spherical(point: Vec3) -> Vec2 {
    let r = point.length();
    let theta = point.z.atan2(point.x);
    let phi = (point.y / r).acos();
    Vec2::new(theta, phi)
}

pub fn spherical_to_cartesian(theta: f32, phi: f32, r: f32) -> Vec3 {
    let x = r * phi.sin() * theta.cos();
    let y = r * phi.cos();
    let z = r * phi.sin() * theta.sin();
    Vec3::new(x, y, z)
}

pub fn normalize_point_to_sphere(point: Vec3, radius: f32) -> Vec3 {
    point.normalize() * radius
}
