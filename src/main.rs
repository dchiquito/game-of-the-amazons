use lazy_static::lazy_static;
use std::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Copy, Debug)]
enum Dim {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
}
impl From<&Dim> for usize {
    fn from(dim: &Dim) -> Self {
        match dim {
            Dim::A => 0,
            Dim::B => 1,
            Dim::C => 2,
            Dim::D => 3,
            Dim::E => 4,
            Dim::F => 5,
            Dim::G => 6,
            Dim::H => 7,
            Dim::I => 8,
            Dim::J => 9,
        }
    }
}
impl From<usize> for Dim {
    fn from(idx: usize) -> Self {
        match idx {
            0 => Dim::A,
            1 => Dim::B,
            2 => Dim::C,
            3 => Dim::D,
            4 => Dim::E,
            5 => Dim::F,
            6 => Dim::G,
            7 => Dim::H,
            8 => Dim::I,
            9 => Dim::J,
            _ => panic!("{idx} is out of bounds"),
        }
    }
}
impl Dim {
    fn less_than(&self) -> Vec<Self> {
        (0..usize::from(self)).rev().map(Self::from).collect()
    }
    fn greater_than(&self) -> Vec<Self> {
        (usize::from(self) + 1..10).map(Self::from).collect()
    }
}

#[derive(Clone, Copy)]
struct Coord(Dim, Dim);
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.0 {
            Dim::A => "a",
            Dim::B => "b",
            Dim::C => "c",
            Dim::D => "d",
            Dim::E => "e",
            Dim::F => "f",
            Dim::G => "g",
            Dim::H => "h",
            Dim::I => "i",
            Dim::J => "j",
        })?;
        f.write_str(match self.1 {
            Dim::A => "1",
            Dim::B => "2",
            Dim::C => "3",
            Dim::D => "4",
            Dim::E => "5",
            Dim::F => "6",
            Dim::G => "7",
            Dim::H => "8",
            Dim::I => "9",
            Dim::J => "10",
        })?;
        Ok(())
    }
}
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
impl From<&Coord> for usize {
    fn from(coord: &Coord) -> Self {
        let col: usize = (&coord.0).into();
        let row: usize = (&coord.1).into();
        (row * 10) + col
    }
}
impl From<usize> for Coord {
    fn from(idx: usize) -> Self {
        assert!(idx < 100);
        let row = idx % 10;
        let col = idx / 10;
        Coord(row.into(), col.into())
    }
}

macro_rules! c {
    ($x:expr) => {{
        let mut s = stringify!($x).chars();
        let row = s.next().expect("Invalid row");
        let col = s.collect::<String>();
        let row = match row {
            'a' => Dim::A,
            'b' => Dim::B,
            'c' => Dim::C,
            'd' => Dim::D,
            'e' => Dim::E,
            'f' => Dim::F,
            'g' => Dim::G,
            'h' => Dim::H,
            'i' => Dim::I,
            'j' => Dim::J,
            _ => panic!("Invalid row"),
        };
        let col = match col.as_ref() {
            "1" => Dim::A,
            "2" => Dim::B,
            "3" => Dim::C,
            "4" => Dim::D,
            "5" => Dim::E,
            "6" => Dim::F,
            "7" => Dim::G,
            "8" => Dim::H,
            "9" => Dim::I,
            "10" => Dim::J,
            _ => panic!("Invalid col"),
        };
        Coord(row, col)
    }};
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TileState {
    Empty,
    White,
    Black,
    Arrow,
}

#[derive(Clone)]
struct Board {
    // Track where the amazons are so we don't need to search the board for them
    pieces: [Coord; 8],
    // Track what state every square on the board is in
    tiles: [TileState; 100],
}

impl Default for Board {
    #[allow(clippy::needless_range_loop)]
    fn default() -> Self {
        // Set up the pieces
        let pieces = [
            c!(a4),
            c!(d1),
            c!(g1),
            c!(j4),
            c!(a7),
            c!(d10),
            c!(g10),
            c!(j7),
        ];
        // Set up an empty board
        let mut tiles = [TileState::Empty; 100];
        // Put the pieces on it
        for w in pieces[0..4].iter() {
            let idx: usize = w.into();
            tiles[idx] = TileState::White;
        }
        for b in pieces[4..8].iter() {
            let idx: usize = b.into();
            tiles[idx] = TileState::Black;
        }
        // Calculate the move lookup table
        Self { pieces, tiles }
    }
}

impl Board {
    pub fn reachable_squares(&self, coord: &Coord) -> ReachableIterator<'_> {
        ReachableIterator::new(self, coord)
    }
    pub fn moves(&self, range: Range<usize>, color: TileState) -> Vec<Board> {
        let mut moves = vec![];
        let mut new_board = self.clone();
        for piece_idx in range {
            // Pick up the piece to clear the path for any arrows fired backward
            new_board.tiles[usize::from(&new_board.pieces[piece_idx])] = TileState::Empty;
            for movement in self.reachable_squares(&self.pieces[piece_idx]) {
                for arrow in new_board.reachable_squares(&movement) {
                    let mut newest_board = new_board.clone();
                    newest_board.pieces[piece_idx] = movement;
                    newest_board.tiles[usize::from(&movement)] = color;
                    newest_board.tiles[usize::from(&arrow)] = TileState::Arrow;
                    moves.push(newest_board);
                }
            }
            // Put the piece back after we've found all of its moves
            new_board.tiles[usize::from(&new_board.pieces[piece_idx])] = color;
        }
        moves
    }
    pub fn white_moves(&self) -> Vec<Board> {
        self.moves(0..4, TileState::White)
    }
    pub fn black_moves(&self) -> Vec<Board> {
        self.moves(4..8, TileState::Black)
    }
    // TODO not useful
    pub fn apply_move(&mut self, piece_index: usize, move_coord: &Coord, arrow_coord: &Coord) {
        // Clear the formerly occupied square
        self.tiles[usize::from(&self.pieces[piece_index])] = TileState::Empty;
        // Move the piece
        self.pieces[piece_index] = *move_coord;
        // Mark the new square the appropriate color
        self.tiles[usize::from(move_coord)] = if piece_index < 4 {
            TileState::White
        } else {
            TileState::Black
        };
        // Place the arrow
        self.tiles[usize::from(arrow_coord)] = TileState::Arrow;
    }
}

struct ReachableIterator<'a> {
    board: &'a Board,
    coord: usize,
    dir: usize,
    idx: usize,
}
impl<'a> ReachableIterator<'a> {
    fn new(board: &'a Board, coord: &Coord) -> Self {
        Self {
            board,
            coord: usize::from(coord),
            dir: 0,
            idx: 0,
        }
    }
}
impl<'a> Iterator for ReachableIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        while self.dir < 8 {
            let moves_in_dir = &MOVES[self.coord][self.dir];
            if self.idx < moves_in_dir.len() {
                let mov = moves_in_dir[self.idx];
                if self.board.tiles[usize::from(&mov)] == TileState::Empty {
                    self.idx += 1;
                    return Some(mov);
                }
            }
            // We have reached the edge of the board or encountered an obstruction
            self.dir += 1;
            self.idx = 0;
        }
        None
    }
}

lazy_static! {
        static ref MOVES: Vec<[Vec<Coord>; 8]> = (0..100)
            .map(|idx| {
                let coord = Coord::from(idx);
                let left = coord.0.less_than();
                let right = coord.0.greater_than();
                let down = coord.1.less_than();
                let up = coord.1.greater_than();
                [
                    // Left
                    left.iter().map(|&x| Coord(x, coord.1)).collect(),
                    // Up+Left
                    left.iter()
                        .zip(up.iter())
                        .map(|(&x, &y)| Coord(x, y))
                        .collect(),
                    // Up
                    up.iter().map(|&y| Coord(coord.0, y)).collect(),
                    // Up+Right
                    right
                        .iter()
                        .zip(up.iter())
                        .map(|(&x, &y)| Coord(x, y))
                        .collect(),
                    // Right
                    right.iter().map(|&x| Coord(x, coord.1)).collect(),
                    // Down+Right
                    right
                        .iter()
                        .zip(down.iter())
                        .map(|(&x, &y)| Coord(x, y))
                        .collect(),
                    // Down
                    down.iter().map(|&y| Coord(coord.0, y)).collect(),
                    // Down+Left
                    left.iter()
                        .zip(down.iter())
                        .map(|(&x, &y)| Coord(x, y))
                        .collect(),
                ]
            })
            .collect();
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("   a b c d e f g h i j\n")?;
        for i in (0..10).rev() {
            f.write_str(format!("{:<2} ", i + 1).as_ref())?;
            for j in 0..10 {
                f.write_str(match self.tiles[(i * 10) + j] {
                    TileState::Empty => ". ",
                    TileState::White => "W ",
                    TileState::Black => "B ",
                    TileState::Arrow => "o ",
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn moves_heuristic(board: &Board) -> i32 {
    let white_moves = board.white_moves().len() as i32;
    let black_moves = board.black_moves().len() as i32;
    white_moves - black_moves
}
// fn random_heuristic() -> i32 {
//     rand::random()
// }

fn main() {
    let board = Board::default();
    println!("{}", board);
    for nb in board.white_moves() {
        println!("{nb}");
    }
    println!("{}", board.white_moves().len());
}
