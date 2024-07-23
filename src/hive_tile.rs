use crate::tile_types::TileType;

#[derive(Clone, Hash)]
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

    pub fn team(&self) -> bool { self.team }

    pub fn tile_type(&self) -> TileType { self.tile_type }
}