use std::{thread, time::Duration};
use std::io::stdout;
use std::io::Write;
use crossterm::{ event::{self, Event, KeyCode, KeyEventKind}, terminal::{enable_raw_mode, disable_raw_mode}, };
const W: usize = 10;
const H: usize = 15;

struct Piece {
    x: i32,
    y: i32,
    blocks: [(i32, i32); 4],
}

struct Board {
    cells: [[u8; W]; H],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[0; W]; H],
        }
    }

    fn render(&self, piece: &Piece) {
        for y in 0..H {
            for x in 0..W {
                let mut is_piece = false;
                for (dx, dy) in piece.blocks {
                    if piece.x + dx == x as i32 && piece.y + dy == y as i32 {
                        is_piece = true;
                    }
                }
                if is_piece {
                    print!("██");
                }
                else if self.cells[y][x] == 1 {
                    print!("██");
                }
                else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, value: u8) {
        self.cells[y][x] = value;
    }

    fn is_occ(&self, piece: &Piece, dx: i32, dy: i32) -> bool {
        for (px, py) in piece.blocks {
            let x = piece.x + px + dx;
            let y = piece.y + py + dy;
            if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 {
                return true;
            }
            if self.cells[y as usize][x as usize] == 1 {
                return true;
            }
        }
        false
    }
    fn can_rot(&self, piece: &Piece) -> bool {
        for (dx, dy) in piece.blocks {
            let x = piece.x + dx;
            let y = piece.y + dy;
            if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 {
                return false;
            }
            if self.cells[y as usize][x as usize] == 1 {
                return false;
            }
        }
        true
    }
 }

impl Piece {
    fn new_sq() -> Self {
        Self {
            x: 4, y: 0, blocks: [(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
    fn rotate(&mut self) {
        for block in &mut self.blocks {
            let (x, y) = *block;
            *block = (y, -x);
        }
    }
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            blocks: self.blocks,
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut piece = Piece::new_sq();
    let mut ml = false;
    let mut mr = false;
    let mut md = false;
    let mut exit = false;
    enable_raw_mode().unwrap();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        board.render(&piece);
        stdout().flush().unwrap();
        
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind != KeyEventKind::Repeat && key_event.kind != KeyEventKind::Release {
                    match key_event.code {
                        KeyCode::Char('r') => {
                            let mut rpiece = piece.clone();
                            rpiece.rotate();
                            if board.can_rot(&rpiece) {
                                piece = rpiece;
                            }
                        }
                        KeyCode::Left => ml = true,
                        KeyCode::Right => mr = true,
                        KeyCode::Down => md = true,
                        KeyCode::Esc => exit = true,
                        _ => {},
                    }
                }
            }
        }
        
        if exit {
            break;
        }
        if ml && !board.is_occ(&piece, -1, 0) {
            piece.x -= 1;
        }
        
        if mr && !board.is_occ(&piece, 1, 0) {
            piece.x += 1;
        }
        
        if md && !board.is_occ(&piece, 0, 1) {
            piece.y += 1;
        }
        
        ml = false;
        mr = false;
        md = false;
        thread::sleep(std::time::Duration::from_millis(200));
        if board.is_occ(&piece, 0, 1) {
            for (dx, dy) in piece.blocks {
                let px = piece.x + dx;
                let py = piece.y + dy;
                if px >= 0 && py >= 0 && px < W as i32 && py < H as i32 {
                    board.set_cell(px as usize, py as usize, 1);
                }
            }
            piece = Piece::new_sq();
            if board.is_occ(&piece, 0, 0) {
                break;
            }
        }
        else {
            piece.y += 1;
        }
    }
    disable_raw_mode().unwrap();
}