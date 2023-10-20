pub trait Draw {
    fn draw(&self, target: &mut glium::Frame, dimensions: [f32; 2], matrix: &nalgebra::Matrix2<f32>);
}
