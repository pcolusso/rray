use wgpu::{VertexAttribute, vertex_attr_array};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3]                 // 0 Float3
}

const VERTEX_DESC: [VertexAttribute; 1] = vertex_attr_array![
    0 => Float3,
];

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &VERTEX_DESC
        }
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Vertex {
            position: [x, y, 0.0],
            // window_size: [640.0, 480.0],
            // seed: 43.0,
            // camera_origin: [0.0, 0.0, 5.0],
            // camera_lower_left: [0.0, 0.0, 0.0],
            // camera_horizontal: [0.0, 0.0, 0.0],
            // camera_lens_radius: 0.0001
        }
    }
}
 