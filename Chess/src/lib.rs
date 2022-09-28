use std::{fmt, ops};
use std::str::FromStr;
use std::iter::FromIterator;

/// Represents the two different teams in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    White,
    Black,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::White => write!(f, "w"),
            Side::Black => write!(f, "b"),
        }
    }
}

///#########################
/// 
///        Side impl
/// 
///#########################

impl Side {
    /// Get the vertical direction in which a pawn on this side moves
    /// (north or south).
    pub fn pawn_dir(self) -> Dir {
        match self {
            Side::White => Dir::N,
            Side::Black => Dir::S,
        }
    }

    /// Get the rank on which a pawn on this side starts the game.
    pub fn pawn_first_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[1],
            Side::Black => BitBoard::RANKS[6],
        }
    }

    /// Get the rank to which a pawn on this side moves to following
    /// it's special two rank first move.
    pub fn pawn_third_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[3],
            Side::Black => BitBoard::RANKS[4],
        }
    }

    /// Get the rank a pawn on this side must be on for it to be able
    /// to promote on it's next move.
    pub fn pawn_promoting_from_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[6],
            Side::Black => BitBoard::RANKS[1],
        }
    }

    /// The rank a pawn on this side will end up on after promoting to
    /// another piece.
    pub fn pawn_promoting_dest_rank(self) -> BitBoard {
        match self {
            Side::White => BitBoard::RANKS[7],
            Side::Black => BitBoard::RANKS[0],
        }
    }
}

/// Type representing a square on a chessboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Dir {
    N, E, S, W,
    NE, SE, SW, NW,
    NNE, NEE, SEE, SSE, SSW, SWW, NWW, NNW, //Knight directions
}

/// Value type wrapping a single integer representing one of the 12
/// different pieces in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Piece {
    WP,
    WN,
    WB,
    WR,
    WQ,
    WK,
    BP,
    BN,
    BB,
    BR,
    BQ,
    BK,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Piece {
    /// Create an iterator traversing over all pieces in order.
    pub fn all() -> impl Iterator<Item = Piece> {
        ALL.iter().cloned()
    }

    /// Create an iterator traversing over all white pieces in order.
    pub fn whites() -> impl Iterator<Item = Piece> {
        WHITE.iter().cloned()
    }

    /// Create an iterator traversing over all black pieces in order.
    pub fn blacks() -> impl Iterator<Item = Piece> {
        BLACK.iter().cloned()
    }

    /// Returns the king which belongs to the given side.
    pub fn king(side: Side) -> Piece {
        match side {
            Side::White => Piece::WK,
            Side::Black => Piece::BK,
        }
    }

    /// Returns the queen which belongs to the given side.
    pub fn queen(side: Side) -> Piece {
        match side {
            Side::White => Piece::WQ,
            Side::Black => Piece::BQ,
        }
    }

    /// Returns the rook belonging to the given side.
    pub fn rook(side: Side) -> Piece {
        match side {
            Side::White => Piece::WR,
            Side::Black => Piece::BR,
        }
    }

    /// Returns the pawn which belongs to the given side.
    pub fn pawn(side: Side) -> Piece {
        match side {
            Side::White => Piece::WP,
            Side::Black => Piece::BP,
        }
    }

    /// Returns a slice containing all pieces belonging to the given side.
    pub fn of(side: Side) -> impl Iterator<Item = Piece> {
        match side {
            Side::White => (&WHITE).iter().cloned(),
            Side::Black => (&BLACK).iter().cloned(),
        }
    }

    /// Returns the side that this piece belongs to.
    pub fn side(self) -> Side {
        if (self as u8) < 6 {
            Side::White
        } else {
            Side::Black
        }
    }

    /// Checks whether this piece is either a white or black pawn.
    pub fn is_pawn(self) -> bool {
        (self as u8) % 6 == 0
    }

    /// Checks whether this piece is either a white or black knight.
    pub fn is_knight(self) -> bool {
        (self as u8) % 6 == 1
    }

    /// Computes the control set for this piece given it's location and the
    /// locations of all the white and black pieces on the board.
    pub fn control(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::CONTROL_FN[self as usize](loc, whites, blacks)
    }

    /// Computes the control set for this piece given it's location on an
    /// empty board.
    pub fn empty_control(self, loc: Square) -> BitBoard {
        self.control(loc, BitBoard::EMPTY, BitBoard::EMPTY)
    }

    /// Computes the set of legal moves for this piece given it's location
    /// and the locations of all the white and black pieces on the board.
    pub fn moves(self, loc: Square, whites: BitBoard, blacks: BitBoard) -> BitBoard {
        Piece::MOVE_FN[self as usize](loc, whites, blacks)
    }

    const CONTROL_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_control,
        knights::control,
        bishops::control,
        rooks::control,
        queens::control,
        kings::control,
        pawns::black_control,
        knights::control,
        bishops::control,
        rooks::control,
        queens::control,
        kings::control,
    ];

    const MOVE_FN: [fn(Square, BitBoard, BitBoard) -> BitBoard; 12] = [
        pawns::white_moves,
        knights::white_moves,
        bishops::white_moves,
        sliding::rooks::white_moves,
        sliding::queens::white_moves,
        kings::white_moves,
        pawns::black_moves,
        knights::black_moves,
        bishops::black_moves,
        rooks::black_moves,
        queens::black_moves,
        kings::black_moves,
    ];
}

/// Constant piece groupings.
const ALL_PIECES: [Piece; 12] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

const WHITE: [Piece; 6] = [
    Piece::WP,
    Piece::WN,
    Piece::WB,
    Piece::WR,
    Piece::WQ,
    Piece::WK,
];

const BLACK: [Piece; 6] = [
    Piece::BP,
    Piece::BN,
    Piece::BB,
    Piece::BR,
    Piece::BQ,
    Piece::BK,
];

// A bitboard is a value type wrapping a 64 bit integer which represents
/// a set of squares on a chess board. Each bit is mapped to a particular
/// square on the board, 0 -> H1, 1 -> G1,..., 8 -> H2,..., 63 -> A8. For
/// example if we know a piece to reside on a particular square we can
/// use a bitboard to to capture the available moves for that piece.
#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Check if this bitboard contains a particular square.
    pub fn contains(self, square: Square) -> bool {
        self.0 & (1u64 << (square as u64)) != 0
    }

    /// Check if this set is a superset of the other.
    pub fn subsumes(self, other: BitBoard) -> bool {
        (other - self).is_empty()
    }

    /// Check if this bitboard is empty, i.e contains no squares.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Check if this bitboard contains at least one square.
    pub fn is_populated(self) -> bool {
        self.0 != 0
    }

    /// Check if the intersection of this bitboard and the other is
    /// non-empty.
    pub fn intersects(self, other: BitBoard) -> bool {
        !(self & other).is_empty()
    }

    /// Computes the number of squares in this bitboard using the
    /// popcount algorithm.
    pub fn size(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn iter(self) -> impl Iterator<Item = Square> {
        self.into_iter()
    }

    /// Finds the first square in this set if it is non-empty.
    pub fn first(self) -> Option<Square> {
        self.into_iter().next()
    }

    /// Returns a bitboard with the least set bit of this bitboard
    /// or nothing if this bitboard is empty.
    pub fn least_set_bit(self) -> BitBoard {
        let trailing = self.0.trailing_zeros();
        if trailing == 64 {
            BitBoard::EMPTY
        } else {
            BitBoard(1u64 << trailing)
        }
    }

    /// Computes the 'cord' between two squares. Imagine a queen sat
    /// on the source square on and empty board. If the queen can move
    /// to the target square then this method returns the set of
    /// squares which the queen slides along to get to this target
    /// square (inclusive of both ends) otherwise the empty bitboard
    /// is returned.
    pub fn cord(source: Square, target: Square) -> BitBoard {
        cords::get_cord(source, target)
    }

    /// The empty bitboard (set of no squares).
    pub const EMPTY: BitBoard = BitBoard(0u64);
    /// The complete bitboard (set of all squares).
    pub const ALL: BitBoard = BitBoard(!0u64);

    /// Array of bitboards represented the eight ranks, ordered 1 to 8.
    pub const RANKS: [BitBoard; 8] = [
        BitBoard(255),
        BitBoard(65280),
        BitBoard(16711680),
        BitBoard(4278190080),
        BitBoard(1095216660480),
        BitBoard(280375465082880),
        BitBoard(71776119061217280),
        BitBoard(18374686479671623680),
    ];

    /// Array of bitboards represented the eight files, ordered H to A.
    pub const FILES: [BitBoard; 8] = [
        BitBoard(72340172838076673),
        BitBoard(144680345676153346),
        BitBoard(289360691352306692),
        BitBoard(578721382704613384),
        BitBoard(1157442765409226768),
        BitBoard(2314885530818453536),
        BitBoard(4629771061636907072),
        BitBoard(9259542123273814144),
    ];
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::EMPTY
    }
}

/// A bitboard is a set of squares and is therefore iterable.
impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIterator;
    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator(self.0)
    }
}

/// A set of squares can be built from an iterator traversing squares.
impl FromIterator<Square> for BitBoard {
    fn from_iter<I: IntoIterator<Item = Square>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

/// We can collect an iterator of bitboards into a single bitboard under
/// the logical OR binary operator on sets.
impl FromIterator<BitBoard> for BitBoard {
    fn from_iter<I: IntoIterator<Item = BitBoard>>(iter: I) -> Self {
        iter.into_iter().fold(BitBoard::EMPTY, |a, b| a | b)
    }
}

/// Operator implementations for bitboards which all use the underlying u64
/// value.
impl ops::Shr<u8> for BitBoard {
    type Output = Self;
    fn shr(self, shift: u8) -> Self {
        BitBoard(self.0 >> shift as u64)
    }
}

impl ops::Shl<u8> for BitBoard {
    type Output = Self;
    fn shl(self, shift: u8) -> Self {
        BitBoard(self.0 << shift as u64)
    }
}

impl ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl ops::Sub<BitBoard> for BitBoard {
    type Output = Self;
    fn sub(self, other: BitBoard) -> Self {
        BitBoard(self.0 & !other.0)
    }
}

impl ops::Sub<Square> for BitBoard {
    type Output = Self;
    fn sub(self, other: Square) -> Self {
        BitBoard(self.0 & !loc(other))
    }
}

impl ops::BitXor<BitBoard> for BitBoard {
    type Output = Self;
    fn bitxor(self, other: BitBoard) -> Self {
        BitBoard(self.0 ^ other.0)
    }
}

impl ops::BitXor<Square> for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Square) -> Self {
        BitBoard(self.0 ^ loc(rhs))
    }
}

impl ops::BitOr<BitBoard> for BitBoard {
    type Output = Self;
    fn bitor(self, other: BitBoard) -> Self {
        BitBoard(self.0 | other.0)
    }
}

impl ops::BitOr<Square> for BitBoard {
    type Output = Self;
    fn bitor(self, other: Square) -> Self {
        BitBoard(self.0 | loc(other))
    }
}

impl ops::BitAnd<BitBoard> for BitBoard {
    type Output = Self;
    fn bitand(self, other: BitBoard) -> Self {
        BitBoard(self.0 & other.0)
    }
}

impl ops::BitAnd<Square> for BitBoard {
    type Output = Self;
    fn bitand(self, other: Square) -> Self {
        BitBoard(self.0 & loc(other))
    }
}

impl ops::BitXorAssign<BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 ^ rhs.0;
    }
}

impl ops::BitXorAssign<Square> for BitBoard {
    fn bitxor_assign(&mut self, rhs: Square) {
        self.0 = self.0 ^ (1u64 << (rhs as u64));
    }
}

impl ops::BitOrAssign<BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 = self.0 | rhs.0;
    }
}

impl ops::BitOrAssign<Square> for BitBoard {
    fn bitor_assign(&mut self, rhs: Square) {
        self.0 = self.0 | (1u64 << (rhs as u64));
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.into_iter().join(", "))
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.into_iter().join(", "))
    }
}

fn loc(sq: Square) -> u64 {
    1u64 << (sq as u64)
}

/// Type representing a square on a chessboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Square {
    H1, G1, F1, E1, D1, C1, B1, A1,
    H2, G2, F2, E2, D2, C2, B2, A2,
    H3, G3, F3, E3, D3, C3, B3, A3,
    H4, G4, F4, E4, D4, C4, B4, A4,
    H5, G5, F5, E5, D5, C5, B5, A5,
    H6, G6, F6, E6, D6, C6, B6, A6,
    H7, G7, F7, E7, D7, C7, B7, A7,
    H8, G8, F8, E8, D8, C8, B8, A8,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        Square::iter()
            .find(|sq| sq.to_string() == lower)
            .ok_or(anyhow!("Cannot parse {} as a Square", s))
    }
}

impl Square {
    /// Return an iterator traversing all squares in order.
    pub fn iter() -> impl Iterator<Item = Square> {
        ALL_SQUARES.iter().cloned()
    }

    /// Retrieve a square by it's corresponding index.
    pub fn from_index(i: usize) -> Square {
        ALL_SQUARES[i]
    }

    /// Return the index of the rank on which this square resides.
    pub const fn rank_index(self) -> usize {
        (self as usize) / 8
    }

    /// Return the index of the file on which this square resides.
    pub const fn file_index(self) -> usize {
        (self as usize) % 8
    }

    /// Return a bitboard representing the rank on which this square
    /// resides.
    pub fn rank(self) -> BitBoard {
        BitBoard::RANKS[self.rank_index()]
    }

    /// Return a bitboard representing the file on which this square
    /// resides.
    pub fn file(self) -> BitBoard {
        BitBoard::FILES[self.file_index()]
    }

    /// 'Lifts' this square to a singleton set of squares.
    pub const fn lift(self) -> BitBoard {
        BitBoard(1u64 << (self as u64))
    }

    /// Finds the next square on a chessboard from this square in a
    /// given direction if it exists.
    pub fn next(self, dir: Dir) -> Option<Square> {
        let dr = match dir {
            Dir::E | Dir::W => 0,
            Dir::N | Dir::NE | Dir::NEE | Dir::NW | Dir::NWW => 1,
            Dir::NNE | Dir::NNW => 2,
            Dir::S | Dir::SE | Dir::SEE | Dir::SW | Dir::SWW => -1,
            Dir::SSE | Dir::SSW => -2,
        };
        let df = match dir {
            Dir::N | Dir::S => 0,
            Dir::W | Dir::NW | Dir::NNW | Dir::SW | Dir::SSW => 1,
            Dir::NWW | Dir::SWW => 2,
            Dir::E | Dir::NE | Dir::NNE | Dir::SE | Dir::SSE => -1,
            Dir::NEE | Dir::SEE => -2,
        };
        let new_rank = (self.rank_index() as i8) + dr;
        let new_file = (self.file_index() as i8) + df;
        if -1 < new_rank && new_rank < 8 && -1 < new_file && new_file < 8 {
            Some(ALL_SQUARES[(8 * new_rank + new_file) as usize])
        } else {
            None
        }
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a set.
    pub fn search(self, dir: Dir) -> BitBoard {
        self.search_vec(dir).into_iter().collect()
    }

    /// Find all squares in a given direction from this square and
    /// returns them as a vector where the squares are ordered in
    /// increasing distance from this square.
    pub fn search_vec(self, dir: Dir) -> Vec<Square> {
        itertools::iterate(Some(self), |op| op.and_then(|sq| sq.next(dir)))
            .skip(1)
            .take_while(|op| op.is_some())
            .map(|op| op.unwrap())
            .collect()
    }

    /// Find all squares in all directions in a given vector and
    /// returns them as a set.
    pub fn search_all(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter().flat_map(|&dir| self.search(dir)).collect()
    }

    /// Find the squares adjacent to this square in all of the
    /// given directions and returns them as a set.
    pub fn search_one(self, dirs: &Vec<Dir>) -> BitBoard {
        dirs.iter()
            .flat_map(|&dir| self.next(dir).into_iter())
            .collect()
    }
}

impl std::ops::Shl<usize> for Square {
    type Output = Square;
    fn shl(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize + rhs)
    }
}

impl std::ops::Shr<usize> for Square {
    type Output = Square;
    fn shr(self, rhs: usize) -> Self::Output {
        Square::from_index(self as usize - rhs)
    }
}

impl std::ops::Not for Square {
    type Output = BitBoard;
    fn not(self) -> Self::Output {
        !self.lift()
    }
}

impl std::ops::BitOr<Square> for Square {
    type Output = BitBoard;
    fn bitor(self, other: Square) -> Self::Output {
        self.lift() | other.lift()
    }
}

impl std::ops::BitOr<BitBoard> for Square {
    type Output = BitBoard;
    fn bitor(self, other: BitBoard) -> Self::Output {
        self.lift() | other
    }
}

impl std::ops::BitAnd<BitBoard> for Square {
    type Output = BitBoard;
    fn bitand(self, other: BitBoard) -> Self::Output {
        self.lift() & other
    }
}

impl std::ops::Sub<BitBoard> for Square {
    type Output = BitBoard;
    fn sub(self, other: BitBoard) -> Self::Output {
        self.lift() - other
    }
}

#[rustfmt::skip]
const ALL_SQUARES: [Square; 64] = [
    Square::H1, Square::G1, Square::F1, Square::E1, Square::D1, Square::C1, Square::B1, Square::A1,
    Square::H2, Square::G2, Square::F2, Square::E2, Square::D2, Square::C2, Square::B2, Square::A2,
    Square::H3, Square::G3, Square::F3, Square::E3, Square::D3, Square::C3, Square::B3, Square::A3,
    Square::H4, Square::G4, Square::F4, Square::E4, Square::D4, Square::C4, Square::B4, Square::A4,
    Square::H5, Square::G5, Square::F5, Square::E5, Square::D5, Square::C5, Square::B5, Square::A5,
    Square::H6, Square::G6, Square::F6, Square::E6, Square::D6, Square::C6, Square::B6, Square::A6,
    Square::H7, Square::G7, Square::F7, Square::E7, Square::D7, Square::C7, Square::B7, Square::A7,
    Square::H8, Square::G8, Square::F8, Square::E8, Square::D8, Square::C8, Square::B8, Square::A8,
];
