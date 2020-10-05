/// 元组结构体，无字段名
#[derive(Debug)]
pub struct Pair(pub f32, pub f32);

/// 单元组结构体只有一个元素时，是newtype模式
pub struct Score(pub u32);

impl Score {
    pub fn pass(&self) -> bool {
        self.0 >= 60
    }
}
