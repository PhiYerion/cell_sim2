use super::chemicals::Chemicals;

#[derive(Debug, Clone, Copy, Default)]
pub struct Inner {
    pub chemicals: Chemicals,
    pub ph: f32,
    pub test: f32,
}

impl Inner {
    pub fn size(&self) -> f32 {
        self.test
    }
}
