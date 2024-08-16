use amazons_core::*;
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    black: bool,
}

fn main() {
    let args = Args::parse();
    let mut input = String::new();
    let stdin = io::stdin();
    let mut board = Board::default();
    loop {
        if !args.black {
            if let Some((mov, new_board)) = heuristic_white(&board, minimax_white_heuristic) {
                println!("{}", mov.notation());
                board = new_board;
            } else {
                println!("Black wins");
                break;
            }
        } else {
            input.clear();
            stdin.read_line(&mut input).expect("Error reading input");
            board.apply_move(&Move::parse_notation(&input).expect("Failed to parse notation"));
        }
        eprintln!("{board}");
        eprintln!("Cheecky little debug: {}", reachable_heuristic(&board));
        if args.black {
            if let Some((mov, new_board)) = heuristic_black(&board, moves_heuristic) {
                println!("{}", mov.notation());
                board = new_board;
            } else {
                println!("White wins");
                break;
            }
        } else {
            input.clear();
            stdin.read_line(&mut input).expect("Error reading input");
            eprintln!("Read line [{input}]");
            board.apply_move(&Move::parse_notation(&input).expect("Failed to parse notation"));
        }
        eprintln!("{board}");
        eprintln!("Cheecky little debug: {}", reachable_heuristic(&board));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_notation() {
        let all_coords: Vec<Coord> = (0..100).map(|idx: usize| Coord::from(idx)).collect();
        let board: Board = Default::default();
        for a in all_coords.iter() {
            for b in all_coords.iter() {
                for c in all_coords.iter() {
                    let mov = Move(*a, *b, *c, board.clone());
                    let notation = mov.notation();
                    let (aa, bb, cc) = Move::parse_notation(&notation).expect("no fails pls");
                    assert_eq!(a, &aa);
                    assert_eq!(b, &bb);
                    assert_eq!(c, &cc);
                }
            }
        }
    }
}
