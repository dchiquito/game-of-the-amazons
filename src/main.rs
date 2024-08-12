use std::fmt::{Display, Formatter, Write};

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

#[derive(Clone, Copy)]
enum TileState {
    Empty,
    White,
    Black,
    Arrow,
}

struct Board {
    // Track where the amazons are so we don't need to search the board for them
    whites: [Coord; 4],
    blacks: [Coord; 4],
    // Track what state every square on the board is in
    tiles: [TileState; 100],
    // Lookup table for valid moves
    moves: Vec<[Vec<Coord>; 8]>,
}

impl Board {
    pub fn reachable_squares(&self, coord: &Coord) {}
}

impl Default for Board {
    #[allow(clippy::needless_range_loop)]
    fn default() -> Self {
        // Set up the pieces
        let whites = [c!(a4), c!(d1), c!(g1), c!(j4)];
        let blacks = [c!(a7), c!(d10), c!(g10), c!(j7)];
        // Set up an empty board
        let mut tiles = [TileState::Empty; 100];
        // Put the pieces on it
        for w in whites.iter() {
            let idx: usize = w.into();
            tiles[idx] = TileState::White;
        }
        for b in blacks.iter() {
            let idx: usize = b.into();
            tiles[idx] = TileState::Black;
        }
        // Calculate the move lookup table
        let moves = (0..100)
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
        Self {
            whites,
            blacks,
            tiles,
            moves,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("   a b c d e f g h i j\n")?;
        for i in (0..10).rev() {
            f.write_str(format!("{:<2} ", i + 1).as_ref())?;
            for j in 0..10 {
                f.write_str(match self.tiles[(j * 10) + i] {
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

fn main() {
    let board = Board::default();
    println!("{}", board);
    let s = c!(f4);
    println!("{s} {}", usize::from(&s));
    println!("{:?}", board.moves[usize::from(&s)]);
}
