mod board;

fn main() {
    let board = board::generate_board();
    let fen = board::generate_fen(&board);
    println!("{}", fen);
}