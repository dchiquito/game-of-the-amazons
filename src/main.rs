use std::fmt::{Display, Formatter, Write};

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
impl Dim {
    pub fn idx(&self) -> usize {
        match self {
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
impl Coord {
    pub fn idx(&self) -> usize {
        (self.0.idx() * 10) + self.1.idx()
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
    // moves: [[Vec<Coord>; 8]; 100],
}

impl Default for Board {
    fn default() -> Self {
        // Set up the pieces
        let whites = [c!(a4), c!(d1), c!(g1), c!(j4)];
        let blacks = [c!(a7), c!(d10), c!(g10), c!(j7)];
        // Set up an empty board
        let mut tiles = [TileState::Empty; 100];
        // Put the pieces on it
        for w in whites.iter() {
            tiles[w.idx()] = TileState::White;
        }
        for b in blacks.iter() {
            tiles[b.idx()] = TileState::Black;
        }
        // Calculate the move lookup table
        Self {
            whites,
            blacks,
            tiles,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0..10).rev() {
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
}
