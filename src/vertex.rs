use crate::vector3::{ Vector3 };

pub struct Vertex {
    pub position: Vector3<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>) -> Vertex {
        Vertex { position }
    }
}
