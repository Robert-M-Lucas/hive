use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::hex_coord::HexCoord;
use crate::hive_tile::{HiveTile, TileType};
use crate::piece_bag::{PieceBag, STARTING_TOTAL};

enum Move {
    Place(TileType, HexCoord),
    Move(HexCoord, HexCoord)
}

pub struct GameState {
    turn_count: usize,
    tiles: HashMap<HexCoord, HiveTile>,
    pieces: (PieceBag, PieceBag),
    queen_location: (Option<HexCoord>, Option<HexCoord>)
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            turn_count: 0,
            tiles: HashMap::with_capacity(STARTING_TOTAL * 2),
            pieces: (PieceBag::new(), PieceBag::new()),
            queen_location: (None, None),
        }
    }

    pub fn print(&self) {
        fn hex_to_actual(hex_coord: &HexCoord) -> (isize, isize) {
            ((hex_coord.x * 2) + hex_coord.y, hex_coord.y)
        }

        let (mut x_min, mut y_min, mut x_max, mut y_max) = (0, 0, 0, 0);

        for loc in self.tiles.keys() {
            let (x, y) = hex_to_actual(loc);
            if x < x_min { x_min = x }
            if x > x_max { x_max = x }
            if y < y_min { y_min = y }
            if y > y_max { y_max = y }
        }

        let mut temp_row = Vec::with_capacity((x_max - x_min) as usize + 1);
        for _ in 0..((x_max - x_min) + 1) {
            temp_row.push(None);
        }
        let mut grid = Vec::with_capacity((y_max - y_min) as usize + 1);
        for _ in 0..((y_max - y_min) + 1) {
            grid.push(temp_row.clone());
        }

        for (loc, tile) in &self.tiles {
            let (x, y) = hex_to_actual(loc);
            grid[(y - y_min) as usize][(x - x_min) as usize] = Some(tile);
        }

        for row in grid {
            for cell in row {
                if let Some(tile) = cell {
                    if tile.team() {
                        cprint!("<c>{}</>", tile.tile_type().character())
                    }
                    else {
                        cprint!("<g>{}</>", tile.tile_type().character())
                    }
                }
                else {
                    print!(" ");
                }
                print!(" ");
            }
            println!();
        }
    }

    pub fn turn(&self) -> bool {
        self.turn_count % 2 == 0
    }

    fn turn_piece_bag(&self) -> &PieceBag {
        if self.turn() {
            &self.pieces.0
        }
        else {
            &self.pieces.1
        }
    }

    fn turn_piece_bag_mut(&mut self) -> &mut PieceBag {
        if self.turn() {
            &mut self.pieces.0
        }
        else {
            &mut self.pieces.1
        }
    }

    fn force_queen(&self) -> bool {
        self.turn_count == 6 || self.turn_count == 7
    }

    fn get_placeable_locations(&self) -> HashSet<HexCoord> {
        let turn = self.turn();

        let mut locations = HashSet::new();

        for (location, tile) in &self.tiles {
            if tile.team() != turn { continue; }

            's_loop: for surrounding in location.surrounding() {
                if locations.contains(&surrounding) { continue; }
                if self.tiles.contains_key(&surrounding) { continue; } // Only need to check z = 0

                for s_surr in surrounding.surrounding() {
                    if let Some(tile) = self.tiles.get(&s_surr) {
                        if tile.top().team() != turn {
                            continue 's_loop;
                        }
                    }
                }

                locations.insert(surrounding);
            }
        }

        locations
    }

    pub fn get_possible_moves(&self) -> Vec<Move> {
        let placeable = self.turn_piece_bag().get_place_options(self.force_queen());

        if self.turn_count == 0 {
            return placeable.into_iter()
                .map(|t| Move::Place(t, HexCoord::new(0, 0))).collect_vec();
        }

        if self.turn_count == 1 {
            return placeable.into_iter()
                .map(|t| Move::Place(t, HexCoord::new(1, 0))).collect_vec();
        }

        let placeable_locations = self.get_placeable_locations();
        let mut moves = Vec::with_capacity(placeable.len() * placeable_locations.len());

        for loc in placeable_locations {
            for tile in &placeable {
                moves.push(Move::Place(*tile, loc.clone()));
            }
        }

        moves
    }

    fn set_queen_location(&mut self, location: HexCoord) {
        if self.turn() {
            self.queen_location.0 = Some(location);
        }
        else {
            self.queen_location.1 = Some(location);
        }
    }

    pub fn apply_move(&mut self, to_move: Move) {
        match to_move {
            Move::Place(tile_type, location) => {
                if matches!(tile_type, TileType::Queen) {
                    self.set_queen_location(location.clone());
                }
                self.turn_piece_bag_mut().use_piece(tile_type);
                self.tiles.insert(location, HiveTile::new(self.turn(), tile_type));
            }
            Move::Move(from, to) => {
                let removed = self.tiles.remove(&from).unwrap();
                if matches!(&removed.tile_type(), TileType::Queen) {
                    self.set_queen_location(to.clone());
                }
                self.tiles.insert(to, removed);
            }
        };
        self.turn_count += 1;
    }

    fn is_broken(&self) -> bool {
        let mut visited = HashSet::with_capacity(self.tiles.len());
        let start = self.tiles.keys().next().unwrap();
        visited.insert(start.clone());
        self.ant(start.clone(), &mut visited);
        debug_assert!(visited.len() <= self.tiles.len());
        visited.len() == self.tiles.len()
    }

    fn ant(&self, location: HexCoord, visited: &mut HashSet<HexCoord>) {
        for surround in location.surrounding() {
            if self.tiles.contains_key(&surround) && visited.insert(surround.clone()) {
                self.ant(surround, visited);
            }
        }
    }

    pub fn score(&self) -> (isize, bool, bool) {
        let mut white_surroundings: isize = 0;
        let mut black_win = false;
        if let Some(queen_location) = &self.queen_location.0 {
            for surround in queen_location.surrounding() {
                if self.tiles.get(&surround).is_some() {
                    white_surroundings += 1;
                }
            }
            if white_surroundings == 6 {
                black_win = true;
            }
        }
        else {
            white_surroundings = 1;
        }

        let mut black_surroundings: isize = 0;
        let mut white_win = false;
        if let Some(queen_location) = &self.queen_location.1 {
            for surround in queen_location.surrounding() {
                if self.tiles.get(&surround).is_some() {
                    black_surroundings += 1;
                }
            }
            if black_surroundings == 6 {
                white_win = true;
            }
        }
        else {
            black_surroundings = 1;
        }

        if self.turn() {
            (black_surroundings - white_surroundings, white_win, black_win)
        }
        else {
            (white_surroundings - black_surroundings, white_win, black_win)
        }
    }
}