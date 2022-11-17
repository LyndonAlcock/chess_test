use std::{ops::{
    Deref, DerefMut}};
use ggez::{
    glam::*,
    graphics::Rect};


use crate::{
    TILE_SIZE,
    board::Board,
    piece_methods::PieceMethods};


const SPRITE_SHEET_WIDTH: usize = 6;
const SPRITE_SHEET_HEIGHT: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct Piece(pub char);


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

impl PieceMethods<Board> for Piece { }

impl Piece{
    fn as_index(self) -> usize{
        match *self {
            'K' => 0, 'Q' => 1, 'B' => 2,
            'N' => 3, 'R' => 4, 'P' => 5,
            'k' => 6, 'q' => 7, 'b' => 8,
            'n' => 9, 'r' => 10, 'p' => 11,
            _ => 12
        }
    }

    pub fn as_src_rect(self) -> Rect{
        let u_x = self.as_index()%SPRITE_SHEET_WIDTH;
        let u_y = self.as_index()/SPRITE_SHEET_WIDTH;
        let w = 1.0 / (SPRITE_SHEET_WIDTH as f32);
        let h = 1.0 / (SPRITE_SHEET_HEIGHT as f32);
        let x = w * (u_x as f32);
        let y = h * (u_y as f32);
        Rect { x, y, w, h}
    }

    pub fn check_bounds(mouse_pos: Vec2, piece_pos: Vec2) -> bool {
        ((mouse_pos.x > piece_pos.x) & (mouse_pos.x < ( piece_pos.x + TILE_SIZE)))&
        ((mouse_pos.y > piece_pos.y) & (mouse_pos.y < ( piece_pos.y + TILE_SIZE)))
    }
}