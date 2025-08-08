use spirv_std::glam::{Vec2, Vec3, Vec4, vec2, vec3};

struct Vertex {
    position: Vec2,
    color: Vec3,
}

impl From<(Vec2, Vec3)> for Vertex {
    fn from(value: (Vec2, Vec3)) -> Self {
        Self {
            position: value.0,
            color: value.1,
        }
    }
}

impl Vertex {
    const fn new(position: Vec2, color: Vec3) -> Self {
        Self { position, color }
    }
}

const VERTICES: [Vertex; 3] = [
    Vertex::new(vec2(0.0, -0.5), vec3(1.0, 0.0, 0.0)),
    Vertex::new(vec2(0.5, 0.5), vec3(0.0, 1.0, 0.0)),
    Vertex::new(vec2(-0.5, 0.5), vec3(0.0, 0.0, 1.0)),
];

pub fn main(vert_id: usize, out_pos: &mut Vec4, out_color: &mut Vec4) {
    *out_pos = Vec4::from((VERTICES[vert_id].position, 0.0, 1.0));
    *out_color = Vec4::from((VERTICES[vert_id].color, 1.0))
}
