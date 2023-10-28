enum Color = {
    White,
    Black
};
enum PieceType = {
    Pawn { canEnPassante : bool }
    Knight,
    Bishop,
    Rook { hasMoved : bool },
    King { hasMoved : bool },
    Queen
};
struct Piece {
    Type: PieceType,
    Color: Color
}
impl Piece {
    fn character(&self) -> char {
        match self.Color {
            White => match {
                
            }
            Black =>
        }
    }
}
type board [[Piece; 8]; 8];
const INITIAL_BOARD: board = []
fn main(){

}