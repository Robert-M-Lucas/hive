use variant_count::VariantCount;
use derive_getters::Getters;

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

#[derive(Getters)]
pub struct HiveTile {
    team: bool,
    tile_type: TileType,
    above: Option<Box<HiveTile>>
}

impl HiveTile {
    pub fn new(team: bool, tile_type: TileType) -> HiveTile {
        HiveTile {
            team,
            tile_type,
            above: None,
        }
    }

    pub fn top(&self) -> &HiveTile {
        if let Some(above) = &self.above {
            above.top()
        }
        else {
            &self
        }
    }
}