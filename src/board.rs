#[derive(Copy, Clone, PartialEq)]
pub enum Pieces {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty
}

#[derive(PartialEq)]
pub enum Color {
    White,
    Black
}

pub struct CastlingRights {
    white_kingside: bool,
    black_kingside: bool,
    white_queenside: bool,
    black_queenside: bool
}

pub struct Board {
    board: [Pieces; 64],
    castling_rights: CastlingRights,
    halfmove_clock: u32,
    side_to_move: Color,
    en_passant_square: String,
    fullmove_counter: u32
}

const MAILBOX: [i8; 120] = [
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
     -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
     -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
     -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
     -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
     -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
     -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
     -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

const MAILBOX64: [u8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

pub fn generate_board() -> Board {
    let mut board_array: [Pieces; 64] = [Pieces::Empty; 64];
    
    for i in 0..64 {
        let rank = 8 - ((i - (i % 8)) / 8);
        let file = (i % 8) + 1;
        if rank == 8 {
            match file {
                1 => board_array[i] = Pieces::BlackRook,
                2 => board_array[i] = Pieces::BlackKnight,
                3 => board_array[i] = Pieces::BlackBishop,
                4 => board_array[i] = Pieces::BlackQueen,
                5 => board_array[i] = Pieces::BlackKing,
                6 => board_array[i] = Pieces::BlackBishop,
                7 => board_array[i] = Pieces::BlackKnight,
                8 => board_array[i] = Pieces::BlackRook,
                _ => panic!("Bad file in board generation!")
            }
        }
        else if rank == 7 {
            board_array[i] = Pieces::BlackPawn;
        }
        else if rank == 2 {
            board_array[i] = Pieces::WhitePawn;
        }
        else if rank == 1 {
            match file {
                1 => board_array[i] = Pieces::WhiteRook,
                2 => board_array[i] = Pieces::WhiteKnight,
                3 => board_array[i] = Pieces::WhiteBishop,
                4 => board_array[i] = Pieces::WhiteQueen,
                5 => board_array[i] = Pieces::WhiteKing,
                6 => board_array[i] = Pieces::WhiteBishop,
                7 => board_array[i] = Pieces::WhiteKnight,
                8 => board_array[i] = Pieces::WhiteRook,
                _ => panic!("Bad file in board generation!")
            }
        }
    }

    Board {
        board: board_array,
        castling_rights: CastlingRights {
            white_kingside: true,
            black_kingside: true,
            white_queenside: true,
            black_queenside: true
        },
        halfmove_clock: 0,
        side_to_move: Color::White,
        en_passant_square: String::from("-"),
        fullmove_counter: 0
    }
}

pub fn generate_fen(board: &Board) -> String {
    let mut fen = String::new();
    let mut empty_counter = 0;
    for i in 0..64 {
        let piece = board.board[i];
        let mut current_piece = String::new();
        match piece {
            Pieces::Empty => empty_counter += 1,
            Pieces::BlackPawn => current_piece = String::from("p"),
            Pieces::BlackKnight => current_piece = String::from("n"),
            Pieces::BlackBishop => current_piece = String::from("b"),
            Pieces::BlackRook => current_piece = String::from("r"),
            Pieces::BlackQueen => current_piece = String::from("q"),
            Pieces::BlackKing => current_piece = String::from("k"),
            Pieces::WhitePawn => current_piece = String::from("P"),
            Pieces::WhiteKnight => current_piece = String::from("N"),
            Pieces::WhiteBishop => current_piece = String::from("B"),
            Pieces::WhiteRook => current_piece = String::from("R"),
            Pieces::WhiteQueen => current_piece = String::from("Q"),
            Pieces::WhiteKing => current_piece = String::from("K"),
        }
        if empty_counter > 0 && piece != Pieces::Empty {
            fen = format!("{}{}{}", fen, empty_counter, current_piece);
        }
        else {
            fen = format!("{}{}", fen, current_piece);
        }
        if i % 8 == 7{
            if empty_counter == 8 {
                fen = format!("{}8", fen);
                empty_counter = 0;
            }
            if i != 63 {
                fen = format!("{}/", fen);
            }
        }
    }

    if board.side_to_move == Color::White {
        fen = format!("{} w", fen);
    }
    else {
        fen = format!("{} w", fen);
    }

    let castling_rights = &board.castling_rights;
    if castling_rights.black_kingside == (castling_rights.white_kingside == (castling_rights.black_queenside == (castling_rights.white_queenside == false))) {
        fen = format!("{} -", fen);
    }
    else {
        fen = format!("{} ", fen);
    }
    if castling_rights.white_kingside {
        fen = format!("{}K", fen);
    }
    if castling_rights.white_queenside {
        fen = format!("{}Q", fen);
    }
    if castling_rights.black_kingside {
        fen = format!("{}k", fen);
    }
    if castling_rights.black_queenside {
        fen = format!("{}q", fen);
    }

    fen = format!("{} {}", fen, board.en_passant_square);

    fen = format!("{} {}", fen, board.halfmove_clock.to_string());

    fen = format!("{} {}", fen, board.fullmove_counter.to_string());

    fen
}