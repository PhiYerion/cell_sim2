use super::chemicals::Chemicals;

pub const PROTEIN_SIZE: f32 = 1.0;
pub const NUCLEOTIDE_SIZE: f32 = 1.0;

#[derive(Debug, Clone, Copy, Default)]
pub struct Inner {
    pub chemicals: Chemicals,
    pub nucleotides: f32,
    pub proteins: f32,
    pub ph: f32,
    pub test: f32,
}

impl Inner {
    pub fn size(&self) -> f32 {
        self.test
    }
}
