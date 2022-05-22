#[derive(Debug)]
pub struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
