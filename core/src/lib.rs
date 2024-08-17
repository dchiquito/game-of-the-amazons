use lazy_static::lazy_static;
use rand::seq::IteratorRandom;
use std::{
    fmt::{Display, Formatter, Write},
    mem::swap,
    ops::Range,
    time::{Duration, SystemTime},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dim {
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Coord(pub Dim, pub Dim);
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
impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let mut s = value.chars();
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
    }
}

macro_rules! c {
    ($x:expr) => {
        Coord::from(stringify!($x).as_ref())
    };
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TileState {
    Empty,
    White,
    Black,
    Arrow,
}

#[derive(Clone, Debug)]
pub struct Move(pub Coord, pub Coord, pub Coord);
impl Move {
    pub fn notation(&self) -> String {
        Self::notation_for(&self.0, &self.1, &self.2)
    }
    pub fn notation_for(piece: &Coord, mov: &Coord, arrow: &Coord) -> String {
        format!("{}-{}/{}", piece, mov, arrow)
    }
    pub fn parse_notation(notation: &str) -> Option<Move> {
        let mut iter = notation.trim().split('-');
        let piece = iter.next()?;
        let remainder = iter.next()?;
        let mut iter = remainder.split('/');
        let mov = iter.next()?;
        let arrow = iter.next()?;
        Some(Move(
            Coord::from(piece),
            Coord::from(mov),
            Coord::from(arrow),
        ))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.notation())
    }
}

#[derive(Clone)]
pub struct Board {
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
    pub fn moves(&self, range: Range<usize>) -> impl Iterator<Item = Move> + '_ {
        MoveIterator::new(self, range)
    }
    pub fn white_moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.moves(0..4)
    }
    pub fn black_moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.moves(4..8)
    }
    pub fn moves_boards(&self, range: Range<usize>) -> impl Iterator<Item = (Move, Board)> + '_ {
        MoveBoardIterator::new(self, range)
    }
    pub fn white_moves_boards(&self) -> impl Iterator<Item = (Move, Board)> + '_ {
        self.moves_boards(0..4)
    }
    pub fn black_moves_boards(&self) -> impl Iterator<Item = (Move, Board)> + '_ {
        self.moves_boards(4..8)
    }

    pub fn apply_move(&mut self, mov: &Move) {
        let Move(piece_coord, move_coord, arrow_coord) = *mov;
        let piece_index = (0..8)
            .find(|idx: &usize| self.pieces[*idx] == piece_coord)
            .expect("No piece to move");
        // Clear the formerly occupied square
        self.tiles[usize::from(&self.pieces[piece_index])] = TileState::Empty;
        // Move the piece
        self.pieces[piece_index] = move_coord;
        // Mark the new square the appropriate color
        self.tiles[usize::from(&move_coord)] = if piece_index < 4 {
            TileState::White
        } else {
            TileState::Black
        };
        // Place the arrow
        self.tiles[usize::from(&arrow_coord)] = TileState::Arrow;
    }
}

pub struct MoveIterator<'a> {
    board: &'a Board,
    range: Range<usize>,
    piece_iterator: PieceMoveIterator<'a>,
}
impl<'a> MoveIterator<'a> {
    fn new(board: &'a Board, mut range: Range<usize>) -> Self {
        let piece_idx = range.next().unwrap();
        Self {
            board,
            range,
            piece_iterator: PieceMoveIterator::new(board, piece_idx),
        }
    }
}
impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mov) = self.piece_iterator.next() {
                return Some(mov);
            } else if let Some(piece_idx) = self.range.next() {
                self.piece_iterator = PieceMoveIterator::new(self.board, piece_idx);
            } else {
                return None;
            }
        }
    }
}
pub struct MoveBoardIterator<'a> {
    board: &'a Board,
    range: Range<usize>,
    piece_iterator: PieceMoveIterator<'a>,
}
impl<'a> MoveBoardIterator<'a> {
    fn new(board: &'a Board, mut range: Range<usize>) -> Self {
        let piece_idx = range.next().unwrap();
        Self {
            board,
            range,
            piece_iterator: PieceMoveIterator::new(board, piece_idx),
        }
    }
}
impl<'a> Iterator for MoveBoardIterator<'a> {
    type Item = (Move, Board);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mov) = self.piece_iterator.next() {
                let mut board = self.board.clone();
                board.apply_move(&mov);
                return Some((mov, board));
            } else if let Some(piece_idx) = self.range.next() {
                self.piece_iterator = PieceMoveIterator::new(self.board, piece_idx);
            } else {
                return None;
            }
        }
    }
}

pub struct PieceMoveIterator<'a> {
    board: &'a Board,
    piece_idx: usize,
    piece_start: usize,
    piece_dir: usize,
    piece_dist: usize,
    arrow_dir: usize,
    arrow_dist: usize,
}
impl<'a> PieceMoveIterator<'a> {
    fn new(board: &'a Board, piece_idx: usize) -> Self {
        Self {
            board,
            piece_idx,
            piece_start: usize::from(&board.pieces[piece_idx]),
            piece_dir: 0,
            piece_dist: 0,
            arrow_dir: 0,
            arrow_dist: 0,
        }
    }
}
impl<'a> Iterator for PieceMoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        // Keep trying directions to start walking
        while self.piece_dir < 8 {
            let moves_in_dir = &MOVES[self.piece_start][self.piece_dir];
            // Keep walking forward until we hit the edge of the board
            while self.piece_dist < moves_in_dir.len() {
                let arrow_start = usize::from(&moves_in_dir[self.piece_dist]);
                if self.board.tiles[arrow_start] == TileState::Empty {
                    // Keep trying directions to fire
                    while self.arrow_dir < 8 {
                        let shots_in_dir = &MOVES[arrow_start][self.arrow_dir];
                        if self.arrow_dist < shots_in_dir.len() {
                            let arrow = shots_in_dir[self.arrow_dist];
                            let arrow = usize::from(&arrow);
                            // We are allowed to shoot through the formerly occupied square
                            if arrow == self.piece_start
                                || self.board.tiles[arrow] == TileState::Empty
                            {
                                // We have a valid move and a valid arrow
                                // Increment the arrows flight distance
                                self.arrow_dist += 1;
                                return Some(Move(
                                    self.board.pieces[self.piece_idx],
                                    Coord::from(arrow_start),
                                    Coord::from(arrow),
                                ));
                            }
                        }
                        // If we have not returned before reaching here, then we have reached the edge
                        // of the board or a encountered a blocking piece or arrow.
                        // Rotate and try a new direction
                        self.arrow_dir += 1;
                        self.arrow_dist = 0;
                    }
                    // If we have reached here without returning, we have done a full 360 with the
                    // arrow.
                    // Time to advance to the next square.
                    self.piece_dist += 1;
                } else {
                    // We have walked into a piece or arrow, stop walking forward.
                    break;
                }
            }
            // If we have reached here without returning, then we have walked to the edge of the
            // board or encountered a blocking piece or arrow.
            // Rotate and try a new direction
            self.piece_dir += 1;
            self.piece_dist = 0;
        }
        // We have done a full 360, there's nothing more to check
        None
    }
}

pub struct ReachableIterator<'a> {
    board: &'a Board,
    coord: usize,
    dir: usize,
    moves_in_dir: &'a Vec<Coord>,
    idx: usize,
}
impl<'a> ReachableIterator<'a> {
    fn new(board: &'a Board, coord: &Coord) -> Self {
        Self {
            board,
            coord: usize::from(coord),
            dir: 0,
            moves_in_dir: &MOVES[usize::from(coord)][0],
            idx: 0,
        }
    }
}
impl<'a> Iterator for ReachableIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        while self.dir < 8 {
            self.moves_in_dir = &MOVES[self.coord][self.dir];
            if self.idx < self.moves_in_dir.len() {
                let mov = self.moves_in_dir[self.idx];
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
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
pub fn random_white(board: &Board) -> Option<Move> {
    board.black_moves().choose(&mut rand::thread_rng())
}
pub fn random_black(board: &Board) -> Option<Move> {
    board.white_moves().choose(&mut rand::thread_rng())
}

pub fn moves_heuristic(board: &Board) -> f64 {
    let white_moves = board.white_moves().count() as f64;
    let black_moves = board.black_moves().count() as f64;
    white_moves - black_moves
}

pub fn area_heuristic(board: &Board) -> i32 {
    let mut white_reachable = [false; 100];
    let mut white_sum = 0;
    for piece in board.pieces[0..4].iter() {
        for reachable in board.reachable_squares(piece) {
            let reachable = usize::from(&reachable);
            if !white_reachable[reachable] {
                white_reachable[reachable] = true;
                white_sum += 1;
            }
        }
    }
    let mut black_reachable = [false; 100];
    let mut black_sum = 0;
    for piece in board.pieces[4..8].iter() {
        for reachable in board.reachable_squares(piece) {
            let reachable = usize::from(&reachable);
            if !black_reachable[reachable] {
                black_reachable[reachable] = true;
                black_sum += 1;
            }
        }
    }
    white_sum - black_sum
}
pub fn reachable_heuristic(board: &Board) -> i32 {
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum By {
        White(usize),
        Black(usize),
        Both,
        Dunno,
    }
    let mut cells = [By::Dunno; 100];
    let mut white_seeds: Vec<usize> = board.pieces[0..4].iter().map(usize::from).collect();
    let mut black_seeds: Vec<usize> = board.pieces[4..8].iter().map(usize::from).collect();
    let mut moves = 1;
    let mut white_cells = 0;
    let mut black_cells = 0;
    while (!white_seeds.is_empty()) && (!black_seeds.is_empty()) {
        let mut new_white_seeds = Vec::with_capacity(20); // TODO what capacity
        for seed in white_seeds.iter() {
            // Check to make sure black didn't get to this spot at the same time
            if cells[*seed] != By::Both {
                for mov in board.reachable_squares(&Coord::from(*seed)) {
                    let mov = usize::from(&mov);
                    if cells[mov] == By::Dunno {
                        cells[mov] = By::White(moves);
                        new_white_seeds.push(mov);
                        white_cells += 1;
                    }
                }
            }
        }
        white_seeds = new_white_seeds;
        let mut new_black_seeds = Vec::with_capacity(20); // TODO what capacity
        for seed in black_seeds.iter() {
            for mov in board.reachable_squares(&Coord::from(*seed)) {
                let mov = usize::from(&mov);
                if cells[mov] == By::Dunno {
                    cells[mov] = By::Black(moves);
                    new_black_seeds.push(mov);
                    black_cells += 1;
                // Now check if black and white get there at the same time
                } else if cells[mov] == By::White(moves) {
                    cells[mov] = By::Both;
                    white_cells -= 1;
                }
            }
        }
        black_seeds = new_black_seeds;
        moves += 1;
    }
    white_cells - black_cells
}

#[allow(clippy::needless_range_loop)]
pub fn better_reachable_heuristic(board: &Board) -> f64 {
    let mut squares = [[0.0; 8]; 100];
    let mut seeds = vec![];
    let mut next_seeds = vec![]; // TODO capacity
    for piece_idx in 0..8 {
        seeds.clear();
        seeds.push(board.pieces[piece_idx]);
        let mut moves = 1.0;
        while !seeds.is_empty() {
            for seed in seeds.iter() {
                for mov in board.reachable_squares(seed) {
                    let mov_idx = usize::from(&mov);
                    if squares[mov_idx][piece_idx] == 0.0 {
                        squares[mov_idx][piece_idx] = moves;
                        next_seeds.push(mov);
                    }
                }
            }
            swap(&mut seeds, &mut next_seeds);
            next_seeds.clear();
            moves += 1.0;
        }
    }
    squares
        .iter()
        .map(|distances| {
            let white_sum: f64 = distances[0..4]
                .iter()
                .map(|d: &f64| if *d != 0.0 { 1.0 / d.powi(2) } else { 0.0 })
                .sum();
            let black_sum: f64 = distances[4..8]
                .iter()
                .map(|d| if *d != 0.0 { 1.0 / d.powi(2) } else { 0.0 })
                .sum();
            if white_sum + black_sum != 0.0 {
                (white_sum - black_sum) / (white_sum + black_sum)
            } else {
                0.0
            }
        })
        .sum()
}

// TODO best reachable heuristic:
// factor in the number of paths to a square
// if you can reach the whole board in 3 moves, but there is only one path to each of those
// squares, then you have a severe choke point.

const HEURISTIC: fn(&Board) -> MMT = better_reachable_heuristic;
// const HEURISTIC: fn(&Board) -> MMT = moves_heuristic;
const TIME_PER_TURN: Duration = Duration::from_secs(10);
#[allow(clippy::upper_case_acronyms)]
type MMT = f64;
fn _minimax(
    board: &Board,
    depth: usize,
    maxing: bool,
    alpha: MMT,
    beta: MMT,
    c: &mut usize,
    timeout: SystemTime,
) -> (Option<(Move, Board)>, MMT) {
    *c += 1;
    let mut alpha = alpha;
    let mut beta = beta;
    if depth == 0 || SystemTime::now() > timeout {
        // if depth == 0 || (*c % POLL_INTERVAL == 0 && SystemTime::now() > timeout) {
        (None, HEURISTIC(board))
    } else if depth > 111111 {
        // TODO disabling this branch temporarily
        if maxing {
            let mut moves: Vec<(Option<(Move, Board)>, MMT)> = board
                .white_moves_boards()
                .map(|(mov, board)| {
                    let (_, mm) = _minimax(&board, depth - 1, !maxing, alpha, beta, c, timeout);
                    alpha = alpha.max(mm);
                    (Some((mov, board)), mm)
                })
                .collect();
            moves.sort_by(|(_, a), (_, b)| a.total_cmp(b));
            moves
                .into_iter()
                .take_while(|(_, mm)| mm <= &beta)
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .unwrap_or((None, MMT::MAX))
        } else {
            let mut moves: Vec<(Option<(Move, Board)>, MMT)> = board
                .black_moves_boards()
                .map(|(mov, board)| {
                    let (_, mm) = _minimax(&board, depth - 1, !maxing, alpha, beta, c, timeout);
                    beta = beta.min(mm);
                    (Some((mov, board)), mm)
                })
                .collect();
            moves.sort_by(|(_, a), (_, b)| a.total_cmp(b));
            moves
                .into_iter()
                .take_while(|(_, mm)| mm >= &alpha)
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .unwrap_or((None, MMT::MIN))
        }
    } else if maxing {
        board
            .white_moves_boards()
            .map(|(mov, board)| {
                let (_, mm) = _minimax(&board, depth - 1, !maxing, alpha, beta, c, timeout);
                alpha = alpha.max(mm);
                (Some((mov, board)), mm)
            })
            .take_while(|(_, mm)| mm <= &beta)
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap_or((None, MMT::MAX))
    } else {
        board
            .black_moves_boards()
            .map(|(mov, board)| {
                let (_, mm) = _minimax(&board, depth - 1, !maxing, alpha, beta, c, timeout);
                beta = beta.min(mm);
                (Some((mov, board)), mm)
            })
            .take_while(|(_, mm)| mm >= &alpha)
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap_or((None, MMT::MIN))
    }
}

pub fn minimax(board: &Board, is_white: bool) -> (Option<(Move, Board)>, MMT) {
    // const POLL_INTERVAL: usize = 1; // how many cycles to go between timer checks
    //                                 // TODO terminate better
    let start_time = SystemTime::now();
    let timeout = start_time + TIME_PER_TURN;
    let mut count = 0;
    let mut depth = 0;
    let mut result = (None, HEURISTIC(board));
    while SystemTime::now() < timeout {
        eprintln!("  calculating depth {depth}");
        let next_result = _minimax(
            board,
            depth,
            is_white,
            MMT::MIN,
            MMT::MAX,
            &mut count,
            timeout,
        );
        if SystemTime::now() < timeout {
            // IF we haven't timed out yet, then we know for sure we completely explored the tree
            // up to the current depth. We don't want to use a partial calculation.
            result = next_result;
        }
        eprintln!("  got {result:?}");
        depth += 1;
    }
    eprintln!("Called minimax {count} times up to depth {depth}");
    eprintln!("Evaluated as {:?}", result.1);
    result
}

pub fn print_h(board: &Board) {
    let mut squares = [[0.0; 8]; 100];
    let mut seeds = vec![];
    let mut next_seeds = vec![]; // TODO capacity
    for piece_idx in 0..8 {
        seeds.clear();
        seeds.push(board.pieces[piece_idx]);
        let mut moves = 1.0;
        while !seeds.is_empty() {
            for seed in seeds.iter() {
                for mov in board.reachable_squares(seed) {
                    let mov_idx = usize::from(&mov);
                    if squares[mov_idx][piece_idx] == 0.0 {
                        squares[mov_idx][piece_idx] = moves;
                        next_seeds.push(mov);
                    }
                }
            }
            swap(&mut seeds, &mut next_seeds);
            next_seeds.clear();
            moves += 1.0;
        }
    }
    let hs: Vec<f64> = squares
        .iter()
        .map(|distances| {
            let white_sum: f64 = distances[0..4]
                .iter()
                .map(|d: &f64| if *d != 0.0 { 1.0 / d.powi(2) } else { 0.0 })
                .sum();
            let black_sum: f64 = distances[4..8]
                .iter()
                .map(|d| if *d != 0.0 { 1.0 / d.powi(2) } else { 0.0 })
                .sum();
            if white_sum + black_sum != 0.0 {
                (white_sum - black_sum) / (white_sum + black_sum)
            } else {
                0.0
            }
        })
        .collect();
    for i in (0..10).rev() {
        for j in 0..10 {
            let coord = (i * 10) + j;
            eprint!("{:6.3} ", hs[coord]);
        }
        eprintln!();
        eprintln!();
    }
    for i in 0..8 {
        eprintln!(
            "{i}: {}",
            squares
                .iter()
                .map(|arr| arr[i])
                .map(|d| if d != 0.0 { 1.0 / d.powi(2) } else { 0.0 })
                .sum::<f64>()
        );
    }
    eprintln!("{}", hs.iter().sum::<f64>());
}
