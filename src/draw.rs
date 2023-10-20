pub trait Draw {
    fn draw(&self, target: &mut glium::Frame, matrix: &nalgebra::Matrix2<f32>);
}
