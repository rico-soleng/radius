use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub const PLANE_VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1., -1.],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1., 1.],
        tex_coords: [0.0, 0.5],
    },
    Vertex {
        position: [1., -1.],
        tex_coords: [0.5, 0.0],
    },
    Vertex {
        position: [1., 1.],
        tex_coords: [0.5, 0.5],
    },
];
