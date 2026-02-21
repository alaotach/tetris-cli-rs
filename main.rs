use std::{thread, time::Duration};
use std::io::stdout;
use std::io::Write;
const W: usize = 10;
const H: usize = 15;

struct Piece {
    x: i32,
    y: i32,
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
                if piece.x == x as i32 && piece.y == y as i32 {
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

    fn is_occ(&self, x:i32, y:i32) -> bool {
        if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 {
            return true;
        }
        self.cells[y as usize][x as usize] == 1
    }
}

fn main() {
    let mut board = Board::new();
    let mut piece = Piece { x: 4, y: 0 };
    loop {
        print!("\x1B[2J\x1B[1;1H");
        board.render(&piece);
        stdout().flush().unwrap();
        thread::sleep(std::time::Duration::from_millis(200));
        if board.is_occ(piece.x, piece.y+1) {
            board.set_cell(piece.x as usize, piece.y as usize, 1);
            piece = Piece { x: 4, y: 0 };
            if board.is_occ(piece.x, piece.y) {
                break;
            }
        }
        else {
            piece.y += 1;
        }
    }
}