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
struct Square {
    piece: Option<Piece>,
}

impl Square {
    pub fn new() -> Square {
        Square {piece: None}
    }

    pub fn symbol(&self) -> &str {
        match self.piece {
            Some(Piece::WKing)   => "K ",
            Some(Piece::WQueen)  => "Q ",
            Some(Piece::WRook)   => "R ",
            Some(Piece::WBishop) => "B ",
            Some(Piece::WKnight) => "N ",
            Some(Piece::WPawn)   => "P ",

            Some(Piece::BKing)   => "k ",
            Some(Piece::BQueen)  => "q ",
            Some(Piece::BRook)   => "r ",
            Some(Piece::BBishop) => "b ",
            Some(Piece::BKnight) => "n ",
            Some(Piece::BPawn)   => "p ",
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
            squares[x][1].piece = Some(Piece::WPawn);
            squares[x][6].piece = Some(Piece::BPawn);
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
            squares[x][0].piece = wpiece;
            squares[x][7].piece = bpiece;
        }
    
        Board {
            squares: squares
        }
    }

    pub fn moves(self, dir: &str) -> Board {
        let mut chessboard = self;
        
        let (from, to) = dir.split_at(2);  

        let (file1, rank1) = from.split_at(1);
        let (file2, rank2) = to.split_at(1);

        let index1 = FILES.iter().position(|&r| r == file1).unwrap();
        let index2 = FILES.iter().position(|&r| r == file2).unwrap();

        let player_p = chessboard.squares[index1][rank1.parse::<usize>().unwrap()].piece;
        let capture_p = chessboard.squares[index2][rank2.parse::<usize>().unwrap()].piece;

        let bp = capture_p.unwrap();
        let wp = player_p.unwrap();
    
        if  WHITE.contains(&bp)  {
            println!("Square is occupied by a white piece!");
            return chessboard;
        }
        if BLACK.contains(&bp) {
            chessboard.squares[index1][rank1.parse::<usize>().unwrap()].piece = None;
            chessboard.squares[index2][rank2.parse::<usize>().unwrap()].piece = player_p;
            return chessboard;
        }





        
        chessboard.squares[index1][rank1.parse::<usize>().unwrap()].piece = None;
        chessboard.squares[index2][rank2.parse::<usize>().unwrap()].piece = player_p;

        
        chessboard

    }

    
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        let mut rank = [" 8", " 7", " 6", " 5", " 4", " 3", " 2", " 1"];
        let mut file = "\nA B C D E F G H";
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

const FILES: &'static [&'static str] = &["h", "g", "f", "e", "d", "c", "b", "a"];

const WHITE:[Piece; 5] = [Piece::WKing , Piece::WQueen , Piece::WBishop , Piece::WKnight , Piece::WPawn];
const BLACK:[Piece; 5] = [Piece::BKing , Piece::BQueen , Piece::BBishop , Piece::BKnight , Piece::BPawn];
