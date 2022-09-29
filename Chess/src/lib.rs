use std::fmt;



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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WKing,
    WQueen,
    WRook,
    WBishop,
    WKnight,
    WPawn,
    BKing,
    BQueen,
    BRook,
    BBishop,
    BKnight,
    BPawn
}

#[derive(Clone, Copy)]
pub struct Square {
    piece: Option<Piece>,
}

impl Square {
    pub fn new() -> Square {
        Square {piece: None}
    }

    pub fn symbol(&self) -> &str {
        match self.piece {
            Some(Piece::WKing)   => "♚ ",
            Some(Piece::WQueen)  => "♛ ",
            Some(Piece::WRook)   => "♜ ",
            Some(Piece::WBishop) => "♝ ",
            Some(Piece::WKnight) => "♞ ",
            Some(Piece::WPawn)   => "♟ ",

            Some(Piece::BKing)   => "♔ ",
            Some(Piece::BQueen)  => "♕ ",
            Some(Piece::BRook)   => "♖ ",
            Some(Piece::BBishop) => "♗ ",
            Some(Piece::BKnight) => "♘ ",
            Some(Piece::BPawn)   => "♙ ",
            None                => ". "
            }
        }
    }

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

pub struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    pub fn new() -> Board {
        let mut squares = [[Square { piece: None }; 8]; 8];
    
        for x in 0..8 {
            squares[x][6].piece = Some(Piece::WPawn);
            squares[x][1].piece = Some(Piece::BPawn);
            let wpiece = match x {
                0 | 7 => Some(Piece::WRook),
                1 | 6 => Some(Piece::WKnight),
                2 | 5 => Some(Piece::WBishop),
                3 => Some(Piece::WKing),
                4 => Some(Piece::WQueen),
                _ => unreachable!()
            };
            let bpiece = match x {
                0 | 7 => Some(Piece::BRook),
                1 | 6 => Some(Piece::BKnight),
                2 | 5 => Some(Piece::BBishop),
                3 => Some(Piece::BKing),
                4 => Some(Piece::BQueen),
                _ => unreachable!()
            };
            squares[x][7].piece = wpiece;
            squares[x][0].piece = bpiece;
        }
    
        Board {
            squares: squares
        }
    }

    pub fn moves(board: Board, dir: &str) -> Board {
        let mut chessboard = board;
        
        let from = Board::get_from(dir);
        let to = Board::get_to(dir);

        let mut from_sq = chessboard.squares[Board::file_index(from.0)][Board::rank_index(from.1)];
        let mut to_sq = chessboard.squares[Board::file_index(to.0)][Board::rank_index(to.1)];

        if (Board::legal_moves(to_sq, from_sq.piece.unwrap(), from, to) == true) {
            if WHITE.contains(&from_sq.piece) { 
                if  WHITE.contains(&to_sq.piece)  {
                    println!("Square is occupied by a white piece!");
                }
                else {
                    println!("{:?}", chessboard.squares[Board::file_index(from.0)][Board::rank_index(from.1)].piece);
                    chessboard.squares[Board::file_index(from.0)][Board::rank_index(from.1)].piece = None;
                    chessboard.squares[Board::file_index(to.0)][Board::rank_index(to.1)].piece = from_sq.piece;
                 } 
            }
            if BLACK.contains(&from_sq.piece) {
                if  BLACK.contains(&to_sq.piece)  {
                    println!("Square is occupied by a white piece!");
                }
                else {
                    chessboard.squares[Board::file_index(from.0)][Board::rank_index(from.1)].piece = None;
                    chessboard.squares[Board::file_index(to.0)][Board::rank_index(to.1)].piece = from_sq.piece;
                 } 
            }
        }
        return chessboard;
    }

    pub fn file_index(file: &str) -> usize {
        return FILES.iter().position(|&rs| rs == file).unwrap(); 
    }

    pub fn rank_index(rank: &str) -> usize {
        return RANKS.iter().position(|&rs| rs == rank.parse::<usize>().unwrap()).unwrap();
    }

    pub fn get_from(dir: &str) -> (&str, &str) {
        let from = dir.split_at(2);  
        return from.0.split_at(1);
           
    }
    pub fn get_to(dir: &str) -> (&str, &str) {
        let to = dir.split_at(2);  
        return to.1.split_at(1);
    }

    pub fn legal_moves(sq: Square, p: Piece, f: (&str, &str), t: (&str, &str)) -> bool {
        let file1 = Board::file_index(t.0) as i32;
        let file2 = Board::file_index(f.0) as i32;
        let rank1 = Board::rank_index(t.1) as i32;
        let rank2 = Board::rank_index(f.1) as i32;

        let file_average = file1 - file2;
        let rank_average = rank1 - rank2;

        println!("{}", rank_average);

        let mut legal = false; 

        if (p == Piece::BPawn) || (p == Piece::WPawn) { legal = Board::pawn_valid(sq, p, file_average, rank_average, f); }
        if (p == Piece::BRook) || (p == Piece::WRook) { legal = Board::rook_valid(sq, p, file_average, rank_average); }
        if (p == Piece::BBishop) || (p == Piece::WBishop) { legal = Board::bishop_valid(sq, p, file_average, rank_average); }
        if (p == Piece::BKnight) || (p == Piece::WKnight) { legal = Board::knight_valid(sq, p, file_average, rank_average); }
        if (p == Piece::BQueen) || (p == Piece::WQueen) { legal = Board::queen_valid(sq, p, file_average, rank_average); }
        if (p == Piece::BKing) || (p == Piece::WKing) { legal = Board::king_valid(sq, p, file_average, rank_average); } 

        println!("{}", legal);

        return legal;
    }

    fn pawn_valid(sq: Square, p: Piece, f: i32, r: i32, from: (&str, &str)) -> bool {
        let mut legals = false;
        if (p == Piece::WPawn) && (BLACK.contains(&sq.piece)) && (f == 1 && r == -1) && (f == -1 && r == -1) { legals = true; }
        if (p == Piece::WPawn) && (Board::rank_index(from.1) == 6) && (r == -2) {  legals = true; }
        if (p == Piece::WPawn) && (r == -1) {  legals = true; }
        if (p == Piece::BPawn) && (WHITE.contains(&sq.piece)) && (f == 1 && r == 1) && (f == -1 && r == 1) {  legals = true;  }
        if (p == Piece::BPawn) && (Board::rank_index(from.1) == 1) && (r == 2) {  legals = true; }
        if (p == Piece::BPawn) && (r == 1) {  legals = true; }
    
        return legals;
    }
    fn rook_valid(sq: Square, p: Piece, f: i32, r: i32) -> bool {
        return ((p == Piece::WRook) || (p == Piece::BRook)) && ((f <=7 && f >= -7 && r == 0) || (r <= 7 && r >= -7 && f == 0));
    }
    fn bishop_valid(sq: Square, p: Piece, f: i32, r: i32) -> bool {
        return ((p == Piece::WBishop) || (p == Piece::BBishop)) && (f == r);
    }
    fn knight_valid(sq: Square, p: Piece, f: i32, r: i32) -> bool {
        return ((p == Piece::WKnight) || (p == Piece::BKnight)) && ((f == 1 && r == 2) || (f == -1 && r == 2) || (f == 1 && r == -2) || (f == -1 && r == -2) || (f == 2 && r == 1) || (f == -2 && r == 1) || (f == 2 && r == -1) || (f == -2 && r == -1));
    }
    fn queen_valid(sq: Square, p: Piece, f: i32, r: i32) -> bool {
        return ((p == Piece::WBishop) || (p == Piece::BBishop)) && ((f == r) || ((f <=7 && f >= -7 && r == 0) || (r <=7 && r >= -7 && f == 0)));
    }
    fn king_valid(sq: Square, p: Piece, f: i32, r: i32) -> bool {
        return ((p == Piece::WKing) || (p == Piece::BKing)) && ((f == r) || (f == r) || ((f <=1 && f >=-1 && r == 0) || (r <=1 && r >= -1 && f == 0)));
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        let mut rank = [" 8", " 7", " 6", " 5", " 4", " 3", " 2", " 1"];
        let mut file = " A B C D E F G H";
        for i in 0..8 {
            for j in 0..8 {
                res.push_str(self.squares[j][i].symbol());
            }
            res.push_str(rank[i]);
            res.push_str("\n");

            
        }
        res.push_str(file);
        write!(f, "{}", res)
    }
}

const FILES: &'static [&'static str] = &["a", "b", "c", "d", "e", "f", "g", "h"];
const RANKS: [usize; 8] = [8,7,6,5,4,3,2,1];

pub const WHITE:[Option<Piece>; 6] = [Some(Piece::WKing) , Some(Piece::WQueen) , Some(Piece::WBishop) , Some(Piece::WKnight), Some(Piece::WRook), Some(Piece::WPawn)];
pub const BLACK:[Option<Piece>; 6] = [Some(Piece::BKing) , Some(Piece::BQueen) , Some(Piece::BBishop) , Some(Piece::BKnight), Some(Piece::BRook), Some(Piece::BPawn)];

