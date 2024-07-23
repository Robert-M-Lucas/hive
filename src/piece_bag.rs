use crate::hive_tile::TileType;

const STARTING_ANTS: usize = 4;
pub const STARTING_TOTAL: usize = 1 + STARTING_ANTS;

pub struct PieceBag {
    queen: bool,
    ants: usize,
}

impl PieceBag {
    pub fn new() -> PieceBag {
        PieceBag {
            queen: true,
            ants: STARTING_ANTS,
        }
    }

    pub fn get_place_options(&self, is_queen_forced: bool) -> Vec<TileType> {
        if is_queen_forced && self.queen {
            return vec![TileType::Queen];
        }

        let mut options = Vec::with_capacity(TileType::VARIANT_COUNT);

        if self.queen {
            options.push(TileType::Queen);
        }
        if self.ants > 0 {
            options.push(TileType::Ant);
        }

        options
    }

    pub fn use_piece(&mut self, tile_type: TileType) {
        match tile_type {
            TileType::Queen => {
                debug_assert!(self.queen);
                self.queen = false;
            }
            TileType::Ant => {
                debug_assert!(self.ants > 0);
                self.ants -= 1;
            }
        }
    }
}