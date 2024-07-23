use std::collections::HashSet;
use itertools::Itertools;
use variant_count::VariantCount;
use crate::game_state::TileStore;
use crate::hex_coord::HexCoord;

#[derive(VariantCount, Clone, Copy, Hash, Eq, PartialEq)]
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

    pub fn from_character(c: char) -> Option<TileType> {
        Some(match c {
            'Q' => TileType::Queen,
            'A' => TileType::Ant,
            _ => return None,
        })
    }

    pub fn get_moves(&self, location: &HexCoord, tiles: &TileStore) -> Vec<HexCoord> {
        match self {
            TileType::Queen => {
                queen_moves(location, tiles)
            }
            TileType::Ant => {
                queen_moves(location, tiles)
            }
        }
    }
}

fn queen_moves(location: &HexCoord, tiles: &TileStore) -> Vec<HexCoord> {
    possible_steps(location, tiles)
}

fn ant_moves(location: &HexCoord, tiles: &TileStore) -> Vec<HexCoord> {
    let first = possible_steps(location, tiles);
    let mut full = HashSet::new();

    for m in first {
        for m2 in possible_steps(&m, tiles) {
            full.insert(m2);
        }
        full.insert(m);
    }
    full.into_iter().collect_vec()
}

fn possible_steps(location: &HexCoord, tiles: &TileStore) -> Vec<HexCoord> {
    let mut steps = Vec::with_capacity(5);

    let surrounding = location.surrounding();
    let occupied = surrounding.iter().map(|l| tiles.contains_key(&l)).collect_vec();

    for (i, loc) in surrounding.into_iter().enumerate() {
        if occupied[i] { continue }
        if occupied[(i + 5) % 6] && occupied[(i + 1) % 6] { continue }
        steps.push(loc);
    }

    steps
}