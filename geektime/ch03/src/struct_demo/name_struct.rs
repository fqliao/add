/// 具名结构体，指定字段名称
#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn euclidean_distance(&self) -> f32 {
        let r = self.x.powf(2.0) + self.y.powf(2.0);
        r.sqrt()
    }
}
