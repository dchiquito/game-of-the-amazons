use amazons_core::*;
use clap::Parser;
use rand::seq::SliceRandom;
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
        eprintln!("Starting whites turn");
        if !args.black {
            let white_moves = board.white_moves();
            if white_moves.is_empty() {
                eprintln!("Black wins");
                break;
            }
            let mov = white_moves.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", mov.notation());
            board = mov.3.clone();
        } else {
            input.clear();
            match stdin.read_line(&mut input) {
                Ok(x) => eprintln!("{x} bytes"),
                Err(_) => todo!("panic it broke"),
            }
            eprintln!("hmhmhmhmh read_line {input}!");
            board.apply_move(&input);
        }
        eprintln!("{board}");
        eprintln!("Starting blacks turn");
        if args.black {
            let black_moves = board.black_moves();
            if black_moves.is_empty() {
                println!("White wins");
                break;
            }
            let mov = black_moves.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", mov.notation());
            board = mov.3.clone();
        } else {
            input.clear();
            stdin.read_line(&mut input).expect("Error reading input");
            board.apply_move(&input);
        }
        eprintln!("{board}");
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
