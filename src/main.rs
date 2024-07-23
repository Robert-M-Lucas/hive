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

fn computer_select(game_state: &mut GameState, global_hash: &mut HashMap<u64, (isize, usize)>) {
    let moves = game_state.get_possible_moves();
    if moves.len() == 0 {
        println!("Passing turn - no moves available");
        game_state.pass();
        return;
    }

    if game_state.turn() {
        let len = moves.len();
        game_state.apply_move(moves.into_iter().nth(thread_rng().gen_range(0..len)).unwrap());
        return;
    }

    let mut counter: u64 = 0;

    const MAX_DEPTH: usize = 6;

    let best_move = moves.into_iter()
        .fold(
            (if game_state.turn() { isize::MIN } else { isize::MAX }, Move::Move(HexCoord::new(0, 0), HexCoord::new(0, 0))), // Move never used
            |(best_score, current_m), m| {
                let turn = game_state.turn();
                game_state.apply_move(m.clone());

                let score = if turn {
                    minimax(game_state, 0, MAX_DEPTH, best_score, isize::MAX, global_hash, &mut counter)
                }
                else {
                    minimax(game_state, 0, MAX_DEPTH, isize::MIN, best_score, global_hash, &mut counter)
                };

                if game_state.turn_count() > 7 {
                    global_hash.insert(game_state.get_hash(), (score, MAX_DEPTH + 1));
                }

                game_state.undo_move(m.clone());

                // LE / GE necessary to prevent default move from being used
                if (game_state.turn() && score >= best_score) || (!game_state.turn() && score <= best_score) {
                    (score, m)
                }
                else {
                    (best_score, current_m)
                }
            }
        ).1;

    println!("Searched: {counter}");

    debug_assert!(
        match &best_move {
            Move::Place(_, _) => { true }
            Move::Move(a, b) => {
                a != b
            }
        }
    );

    game_state.apply_move(best_move);

    println!("Score: {}", game_state.score().0);
}

fn get_score(score: (isize, bool, bool)) -> isize {
    if score.1 && score.2 {
        return 0;
    }
    if score.1 {
        return isize::MAX;
    }
    else if score.2 {
        return isize::MIN;
    }
    score.0
}

fn minimax(current_state: &mut GameState, depth: usize, max_depth: usize, mut alpha: isize, mut beta: isize, global_hash: &mut HashMap<u64, (isize, usize)>, counter: &mut u64) -> isize {
    *counter += 1;
    let turn = current_state.turn();
    let moves = current_state.get_possible_moves();

    if moves.len() == 0 {
        current_state.pass();
        let hash = current_state.get_hash();
        let found = global_hash.get(&hash);
        let score = if found.is_some() && found.unwrap().1 >= max_depth - depth {
            found.unwrap().0
        }
        else {
            if depth == max_depth {
                // Evaluate board at final depth
                get_score(current_state.score())
            }
            else {
                let score = minimax(current_state, depth + 1, max_depth, alpha, beta, global_hash, counter);
                if current_state.turn_count() > 7 {
                    global_hash.insert(hash, (score, max_depth - depth));
                }
                score
            }
        };
        current_state.unpass();

        return score;
    }

    let mut best = if current_state.turn() { isize::MIN } else { isize::MAX };
    for m in moves.into_iter() {
        current_state.apply_move(m.clone());

        let hash = current_state.get_hash();
        let found = global_hash.get(&hash);
        let score = if found.is_some() && found.unwrap().1 >= max_depth - depth {
            found.unwrap().0
        }
        else {
            if depth == max_depth {
                // Evaluate board at final depth
                get_score(current_state.score())
            }
            else {
                let score = minimax(current_state, depth + 1, max_depth, alpha, beta, global_hash, counter);
                if current_state.turn_count() > 7 {
                    global_hash.insert(hash, (score, max_depth - depth));
                }
                score
            }
        };

        current_state.undo_move(m.clone());

        if turn {
            if score == isize::MAX { return score; }
            best = best.max(score);
            alpha = best.max(score);
        }
        else {
            if score == isize::MIN { return score; }
            best = best.min(score);
            beta = beta.min(score);
        }

        if beta <= alpha {
            break;
        }
    }

    best
}

fn main() {
    let mut global_hash_scores = HashMap::new();

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

        println!("Turn: {} [{}]\n", if game.turn() { "Cyan" } else { "Green" }, game.turn_count());

        // stdout().flush().unwrap();
        // stdin().read_line(&mut String::new()).unwrap();

        println!("Working...");

        if game.turn() {
            computer_select(&mut game, &mut global_hash_scores);
        }
        else {
            computer_select(&mut game, &mut global_hash_scores);
        }
    }
}