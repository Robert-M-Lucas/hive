use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use itertools::Itertools;
use rand::{random, Rng, thread_rng};
use game_state::GameState;
use crate::game_state::Move;
use crate::hex_coord::HexCoord;
use crate::tile_types::TileType;

mod hex_coord;
mod hive_tile;
mod piece_bag;
mod game_state;
mod tile_types;

fn player_select(game_state: &mut GameState) {
    let moves = game_state.get_possible_moves();
    if moves.len() == 0 {
        println!("Passing turn - no moves available");
        game_state.pass();
        return;
    }

    let mut movements = Vec::new();
    let mut placements = Vec::new();
    for m in moves {
        match m {
            Move::Place(tile, to) => {
                placements.push((tile, to));
            }
            Move::Move(from, to) => {
                movements.push((from, to));
            }
        }
    }

    let mut first_loop = true;
    let mut error = None;

    loop {
        if !first_loop {
            game_state.print();
            println!();
            if let Some(e) = error {
                println!("Error: {e}");
                error = None;
            }
        }
        first_loop = false;

        print!("To Place: ");
        game_state.turn_piece_bag().print(game_state.turn());

        if placements.is_empty() {
            println!("X| No placements available");
        }
        else {
            println!("1| Place piece");
        }

        if movements.is_empty() {
            println!("X| No movements available");
        }
        else {
            println!("2| Move piece");
        }

        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let Ok(c) = input.trim().parse::<usize>() else {
            error = Some("Invalid selection");
            continue;
        };

        fn coordinate_parser(input: &str) -> Option<(usize, usize)> {
            let input = input.trim();
            let mut split = input.split(' ');
            let Some(x) = split.next().and_then(|s| s.parse().ok()) else { return None };
            let Some(y) = split.next().and_then(|s| s.parse().ok()) else { return None };
            if split.next().is_some() { return None; }
            Some((x, y))
        }

        if c == 1 && !placements.is_empty() {
            println!("Select piece to place");
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input.chars().count() != 1 {
                error = Some("Invalid piece name");
                continue;
            }
            let c = input.chars().next().unwrap().to_ascii_uppercase();
            let Some(piece) = TileType::from_character(c) else {
                error = Some("Invalid piece name");
                continue;
            };
            if !placements.iter().any(|(t, _)| t == &piece) {
                error = Some("No available placements for given piece");
                continue;
            }

            if game_state.turn_count() == 0 {
                game_state.apply_move(Move::Place(piece, HexCoord::new(0, 0)));
                return;
            }
            else if game_state.turn_count() == 1 {
                game_state.apply_move(Move::Place(piece, HexCoord::new(1, 0)));
                return;
            }

            println!("Enter coordinates to place at - 'x y' e.g. '3 1'");
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let Some((x, y)) = coordinate_parser(&input) else {
                error = Some("Invalid coordinates");
                continue;
            };

            let bounds = game_state.get_square_bounds();
            let x = x as isize + bounds.0 - 1;
            let y = y as isize + bounds.1 - 1;
            let Some(hex_coordinates) = HexCoord::try_from_square(x, y) else {
                error = Some("Invalid coordinates");
                continue;
            };

            if !placements.iter().any(|(t, p)| t == &piece && p == &hex_coordinates) {
                error = Some("Piece cannot be placed at specified location");
                continue;
            }
            game_state.apply_move(Move::Place(piece, hex_coordinates));
            return;
        }
        else if c == 2 && !movements.is_empty() {
            println!("Enter coordinates to move from - 'x y' e.g. '3 1'");
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let Some((x, y)) = coordinate_parser(&input) else {
                error = Some("Invalid coordinates");
                continue;
            };
            let bounds = game_state.get_square_bounds();
            let x = x as isize + bounds.0 - 1;
            let y = y as isize + bounds.1 - 1;
            let Some(from_coords) = HexCoord::try_from_square(x, y) else {
                error = Some("Invalid coordinates");
                continue;
            };
            if !movements.iter().any(|(f, _)| f == &from_coords) {
                error = Some("No valid moves originating from the given position");
                continue;
            }

            println!("Enter coordinates to move to - 'x y' e.g. '3 1'");
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let Some((x, y)) = coordinate_parser(&input) else {
                error = Some("Invalid coordinates");
                continue;
            };
            let bounds = game_state.get_square_bounds();
            let x = x as isize + bounds.0 - 1;
            let y = y as isize + bounds.1 - 1;
            let Some(to_coords) = HexCoord::try_from_square(x, y) else {
                error = Some("Invalid coordinates");
                continue;
            };
            if !movements.iter().any(|(_, t)| t == &to_coords) {
                error = Some("Invalid move");
                continue;
            }

            game_state.apply_move(Move::Move(from_coords, to_coords));
            return;
        }
    }
}

fn computer_select(game_state: &mut GameState) {
    let moves = game_state.get_possible_moves();
    if moves.len() == 0 {
        println!("Passing turn - no moves available");
        game_state.pass();
        return;
    }

    // let len = moves.len();
    // game_state.apply_move(moves.into_iter().nth(thread_rng().gen_range(0..len)).unwrap());


}

fn minimax(mut current_state: GameState, maximising: bool, depth: usize, max_depth: usize, max: isize, min: isize) -> (isize, Option<Move>) {
    let moves = current_state.get_possible_moves();

    if moves.len() == 0 {

    }

    let mut best = None;
    let mut worst = None;
    for (i, m) in moves.into_iter().enumerate() {
        let mut new_state = current_state.clone();
        new_state.apply_move(m);
        let score = if depth == max_depth {
            new_state.score().0
        }
        else {
            minimax(new_state, maximising, depth + 1, max_depth, best, worst).0
        };
        if let Some((c_b, _)) = best {
            if score > c_b {
                best = Some((score, i));
            }
        }
        else {
            best = Some((score, i));
        }

        if let Some((c_w, _)) = worst {
            if score < c_w {
                worst = Some((score, i));
            }
        }
        else {
            worst = Some((score, i));
        }
    }


}

fn main() {
    let mut game = GameState::new();
    loop {
        game.print();

        let (score, w, b) = game.score();

        if w && b {
            println!("Draw!");
            break;
        }
        if w {
            println!("Cyan wins!");
            break;
        }
        if b {
            println!("Green wins!");
            break;
        }

        println!("Turn: {}\n", if game.turn() { "Cyan" } else { "Green" });

        if game.turn() {
            computer_select(&mut game);
        }
        else {
            computer_select(&mut game);
        }
    }
}