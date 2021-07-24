use crate::vector2::{ Vector2 };
use crate::vector3::{ Vector3 };

pub struct Vertex {
    pub position: Vector3<f32>,
    pub texture_coords: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, texture_coords: Vector2<f32>) -> Vertex {
        Vertex {
            position,
            texture_coords,
        }
    }
}
