const W: usize = 10;
const H: usize = 15;

struct Board {
    cells: [[u8; W]; H],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[0; W]; H],
        }
    }

    fn render(&self) {
        for y in 0..H {
            for x in 0..W {
                if self.cells[y][x] == 0 {
                    print!(". ");
                }
                else {
                    print!("██");
                }
            }
            println!();
        }
    }
}

fn main() {
    let board = Board::new();
    board.render();
}