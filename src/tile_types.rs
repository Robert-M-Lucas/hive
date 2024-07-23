use variant_count::VariantCount;

#[derive(VariantCount, Clone, Copy)]
pub enum TileType {
    Queen,
    Ant,
}

impl TileType {
    pub fn character(&self) -> char {
        match self {
            TileType::Queen => 'Q',
            TileType::Ant => 'A'
        }
    }
}