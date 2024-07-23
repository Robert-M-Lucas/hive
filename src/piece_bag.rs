use color_print::cprint;
use crate::tile_types::TileType;

const STARTING_ANTS: usize = 6;
pub const STARTING_TOTAL: usize = 1 + STARTING_ANTS;

#[derive(Clone)]
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

    pub fn print(&self, turn: bool) {
        let q = if self.queen { 1 } else { 0 };
        if turn {
            cprint!("<c>Q</>:{} ", q);
        }
        else {
            cprint!("<g>Q</>:{} ", q);
        }

        if turn {
            cprint!("<c>A</>:{} ", self.ants);
        }
        else {
            cprint!("<g>A</>:{} ", self.ants);
        }

        println!();
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

    pub fn unuse_piece(&mut self, tile_type: TileType) {
        match tile_type {
            TileType::Queen => {
                debug_assert!(!self.queen);
                self.queen = true;
            }
            TileType::Ant => {
                self.ants += 1;
            }
        }
    }
}