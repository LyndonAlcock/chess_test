use std::{path::PathBuf, env};

use board::Board;
use ggez::{
    event::{self, MouseButton, EventHandler},
    glam::*,
    graphics::{self, Image, DrawParam, DrawMode, Mesh, Rect, Color},
    Context, GameResult, GameError,
};
use piece::Piece;

mod board;
mod piece;
mod piece_methods;

use crate::piece_methods::PieceMethods;

const STARTING_FEN : &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const TILE_SIZE : f32 = 80.0;

struct BoardState{
    image: Image,
    mov_rect: Mesh,
    board: Board,
    held_index: IVec2,
    draw_pos: Vec2,
    handle: Vec2
}


impl BoardState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::from_path(ctx, "/pieces.png")?;

        let mov_rect = Mesh::new_rectangle(
            ctx, 
            DrawMode::fill(), 
            Rect::new(0.0,0.0,TILE_SIZE, TILE_SIZE), 
            Color::RED)?;

        Ok( BoardState{
            image,
            mov_rect,
            board: Board::parse_fen(STARTING_FEN),
            held_index: IVec2::NEG_ONE,
            draw_pos : Vec2::ZERO,
            handle: Vec2::ZERO
        } )
            
    }
}


impl EventHandler<GameError> for BoardState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_pos.x = ctx.mouse.position().x + self.handle.x;
        self.draw_pos.y = ctx.mouse.position().y + self.handle.y;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
            
        for v in self.board.get_moves(self.held_index){
            let pos = v.as_vec2() * TILE_SIZE;
            canvas.draw(&self.mov_rect, pos);
        }

        for (i, p) in self.board.pieces.iter().enumerate(){
            let piece_index = ivec2((i%8) as i32, (i/8) as i32);
            let piece_pos = piece_index.as_vec2() * TILE_SIZE;
            let draw_param = DrawParam::new()
                .src(p.as_src_rect())
                .scale(Vec2::splat(0.1));

            if piece_index==self.held_index && !p.is_empty(){
                canvas.draw(&self.image, draw_param
                    .dest(self.draw_pos));
            } else {
                canvas.draw(&self.image, draw_param
                    .dest(piece_pos));
            }
        }


        canvas.finish(ctx)?;
        Ok(())
    }
    

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) -> GameResult {
        
        for (i, _p) in self.board.pieces.iter_mut().enumerate(){
            let piece_index = ivec2((i%8) as i32, (i/8) as i32);
            let piece_pos = piece_index.as_vec2() * TILE_SIZE;
            let mouse_pos = vec2(x, y);
            //let dist = ((x-piece_pos.x).powf(2.) + (y-piece_pos.y).powf(2.)).sqrt();
        
            if  Piece::check_bounds(mouse_pos, piece_pos) & (button == MouseButton::Left){
                self.held_index = piece_index;
                self.handle = vec2(piece_pos.x - x, piece_pos.y - y);
            }
            
        }
        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        
        let drop_index = (vec2(x, y)/TILE_SIZE).floor().as_ivec2();
        if button == MouseButton::Left{
            self.board.move_piece(self.held_index, drop_index);
            self.held_index = IVec2::NEG_ONE;
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = BoardState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}