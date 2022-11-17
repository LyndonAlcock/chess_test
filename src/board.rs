use ggez::glam::*;
use std::{ops::{Index, Deref, DerefMut, IndexMut}, char};
const WIDTH:i32 = 8;
static mut NULL_PIECE:Piece = Piece('\0');
pub struct Board{pub pieces: [Piece;64]}

#[derive(Clone, Copy, Debug)]
pub struct Piece(pub char);

impl PartialEq<char> for Piece {
    fn eq(&self, other: &char) -> bool {
        self.0 == *other
    }
}

impl Deref for Piece {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Piece {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<IVec2> for Board {
    type Output = Piece;
    fn index(&self, index : IVec2) -> &Self::Output{
        if (index.abs() != index) || (index.max_element() > WIDTH-1) {
            &Piece('\0')
        } else {
            let i : usize = (index.x + WIDTH* index.y).try_into().unwrap();
            &self.pieces[i]
        }
    }
}

impl IndexMut<IVec2> for Board {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output{
        if (index.abs() != index) || (index.max_element() > WIDTH-1) {
            unsafe {
                &mut NULL_PIECE
            }
        } else {
            let i : usize = (index.x + WIDTH * index.y).try_into().unwrap();
            &mut self.pieces[i]
        }
    }
}
//pub trait PieceMethods where Self: 
//   PartialEq<char> + Deref<Target = char>

impl Piece
{
    pub fn is_empty(self)-> bool {
        if *self == ' ' {
            true
        } else {
            false
        }
    }
    fn is_black(self)-> bool {
        match *self {
            'k' | 'q' | 'b' | 'n' | 'r' | 'p' => true,
            _=> false
        }
    }
    fn is_white(self)-> bool {
        match *self {
            'K' | 'Q' | 'B' | 'N' | 'R' | 'P' => true,
            _=> false
        }
    }
    fn is_rival(self, other: Self)-> bool {
        if self.is_black() && other.is_white() {true} 
        else if self.is_white() && other.is_black() {true}
        else {false}
    }
    fn is_ally(&self, other: Self)-> bool {
        if self.is_black() && other.is_black() {true} 
        else if self.is_white() && other.is_white() {true}
        else {false}
    }
    fn get_directions(self) -> Vec<IVec2> {
        match *self{
            'K'|'k'|'Q'|'q' => vec![
                ivec2(-1, -1), ivec2( 0, -1), ivec2( 1, -1), ivec2(-1,  0),
                ivec2( 1,  0), ivec2(-1,  1), ivec2( 0,  1), ivec2( 1,  1)],
            'N'|'n' => vec![
                ivec2(-1, -2), ivec2( 1, -2), ivec2(-2, -1), ivec2( 2, -1),
                ivec2(-2,  1), ivec2( 2,  1), ivec2(-1,  2), ivec2( 1,  2)
            ],
            'R'|'r' => vec![
                ivec2( 0, -1), ivec2(-1,  0), ivec2( 1,  0), ivec2( 0,  1)
            ],
            'B'|'b' => vec![
                ivec2(-1, -1), ivec2( 1, -1), ivec2(-1,  1), ivec2( 1,  1)
            ],
            'P' => vec![ivec2(-1, -1), ivec2( 0, -1), ivec2( 1, -1)],
            'p' => vec![ivec2(-1,  1), ivec2( 0,  1), ivec2( 1,  1)],
            _=> Vec::new()
        }
    }
    fn long_move(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2> {
        let next = pos + dir;
        if board[next].is_empty() {
            let mut moves = self.long_move(next, dir, board);
            moves.push(next);
            moves
        } else if board[next].is_rival(self) { 
            vec![next]
        } else {
            vec![]
        }
    }
    fn short_move(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2> {
        let next = pos + dir;
        if board[next].is_empty() | board[next].is_rival(self) { 
            vec![next] 
        } else {
            vec![]
        }
    }
    fn pawn_move(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2> {
        match dir.abs().x{
            0 => { self.short_move(pos, dir, board) },
            1 => { 
                let next = pos + dir;
                    if board[next].is_empty() | board[next].is_rival(self) { 
                    vec![next] 
                } else {
                vec![]
                }
            },
            _ => vec![]
        }
    }
}



impl Board {
    pub fn get_moves(self, pos:IVec2)-> Vec<IVec2>{
        let piece = self[pos];
        piece
            .get_directions()
            .into_iter()
            .flat_map(|dir| {
                match *piece {
                    'K' | 'k' | 'N' | 'n' => piece.short_move(pos, dir, &self),
                    'Q' | 'q' | 'B' | 'b' | 'R' | 'r' => piece.long_move(pos, dir, &self),
                    'P' | 'p' => piece.pawn_move(pos, dir, &self),
                    _ => vec![]
                }
            })
            .collect()
    }

    pub fn move_piece(&mut self, from: IVec2, to: IVec2) {
        *self[to] = self[from].0;
        *self[from] = ' ';
    }

    pub fn parse_fen(s: &str)-> Board{
        let mut ch_v = vec![];
        s.replace("/","")
            .chars()
            .for_each(|c| 
                if c.is_numeric(){
                    for _ in 0..c.to_digit(10).unwrap()
                        {ch_v.push(Piece(c));}}
                else {ch_v.push(Piece(c));});
        Board {pieces: ch_v.try_into().unwrap() }
    }
}

