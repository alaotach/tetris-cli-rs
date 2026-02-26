use std::{thread, io};
use std::time::{Instant, Duration};
use crossterm::{ 
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute };
use ratatui::{ backend::CrosstermBackend, Terminal, layout::{Layout, Constraint, Direction}, widgets::{Block, Borders, Paragraph}, text::{Line, Span}, style::{Color, Style} };
use rand::RngExt;
const W: usize = 10;
const H: usize = 15;

#[derive(Clone, Copy)]
enum Tetromino {I,O,T,L,J,S,Z,}

#[derive(Clone, Copy)]
struct Piece {
    kind: Tetromino,
    x: i32,
    y: i32,
    blocks: [(i32, i32); 4],
    color: Color,
}

struct Board {
    cells: [[Option<Color>; W]; H],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[None; W]; H],
        }
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, piece: &Piece, ghost: &Piece, score: u32, show_ghost: bool) {
        terminal.draw(|f| {
            let size = f.area();
            let wb = (W*2+2) as u16;
            let hb = (H+2) as u16;
            let sw = 28;
            let vert = Layout::default().direction(Direction::Vertical).constraints([
                Constraint::Min(0),
                Constraint::Length(hb),
                Constraint::Min(0),
            ]).split(size);
            let horiz = Layout::default().direction(Direction::Horizontal).constraints([
                Constraint::Min(0),
                Constraint::Length(wb),
                Constraint::Length(sw),
                Constraint::Min(0),
            ]).split(vert[1]);
            let ba = horiz[1]; //board area
            let ia = horiz[2]; //info area
            // let chunks = Layout::default().direction(Direction::Horizontal).constraints([
            //         Constraint::Percentage(30),
            //         Constraint::Percentage(70),
            //     ]).split(size);
            let mut lines = Vec::new();

            for y in 0..H {
                let mut spans = Vec::new();
                for x in 0..W {
                    let mut is_piece = false;
                    let mut is_ghost = false;
                    
                    for (dx, dy) in piece.blocks {
                        if piece.x + dx == x as i32 && piece.y + dy == y as i32 {
                            is_piece = true;
                        }
                    }

                    if show_ghost {
                        for (dx, dy) in ghost.blocks {
                            if ghost.x + dx == x as i32 && ghost.y + dy == y as i32 {
                                is_ghost = true;
                            }
                        }
                    }
                    if is_piece {
                        spans.push(Span::styled("██", Style::default().fg(piece.color)));
                    } else if is_ghost {
                        spans.push(Span::styled("░░", Style::default().fg(piece.color)));
                    } else if let Some(color) = self.cells[y][x] {
                        spans.push(Span::styled("██", Style::default().fg(color)));
                    } else {
                        spans.push(Span::raw("  "));
                    }
                }
                lines.push(Line::from(spans));
            }
            
            let board = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Tetris"));
            f.render_widget(board, ba);
            let info = Paragraph::new(vec![
                Line::from(format!("Score: {}", score)),
                Line::from(""),
                Line::from("Controls:"),
                Line::from("← →/A-D Move"),
                Line::from("↓/S Soft Drop"),
                Line::from("Space Hard Drop"),
                Line::from("r Rotate"),
                Line::from("g Ghost Toggle"),
                Line::from("Esc Exit"),
            ]).block(Block::default().borders(Borders::ALL).title("Info"));
            f.render_widget(info, ia);
        }).unwrap();
    }

    fn set_cell(&mut self, x: usize, y: usize, color: Color) {
        self.cells[y][x] = Some(color);
    }

    fn is_occ(&self, piece: &Piece, dx: i32, dy: i32) -> bool {
        for (px, py) in piece.blocks {
            let x = piece.x + px + dx;
            let y = piece.y + py + dy;
            if x < 0 || x >= W as i32 || y < 0 || y >= H as i32 {
                return true;
            }
            if self.cells[y as usize][x as usize].is_some() {
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
            if self.cells[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }
    fn line_full(&self, y: usize) -> bool {
        for x in 0..W {
            if self.cells[y][x].is_none() {
                return false;
            }
        }
        true
    }
    fn clear_lines(&mut self) -> u32 {
        let mut ngrid = [[None; W]; H];
        let mut nr = H as i32 - 1;
        let mut cleared = 0;
        for y in (0..H).rev() {
            if !self.line_full(y) {
                ngrid[nr as usize] = self.cells[y];
                nr -= 1;
            }
            else {
                cleared += 1;
            }
        }
        self.cells = ngrid;
        cleared
    }
 }

impl Piece {
    fn new(kind: Tetromino) -> Self {
        Self {
            kind,
            x: 4, y: 0, blocks: Piece::shape(kind), color: Piece::color(),
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
            kind: self.kind,
            x: self.x,
            y: self.y,
            blocks: self.blocks,
            color: self.color,
        }
    }
    fn shape(kind: Tetromino) -> [(i32, i32); 4] {
        match kind {
            Tetromino::I => [(0, 0), (1, 0), (2, 0), (3, 0)],
            Tetromino::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Tetromino::T => [(0, 0), (-1, 1), (0, 1), (1, 1)],
            Tetromino::L => [(0, 0), (0, 1), (0, 2), (1, 2)],
            Tetromino::J => [(0, 0), (0, 1), (0, 2), (-1, 2)],
            Tetromino::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            Tetromino::Z => [(0, 0), (-1, 0), (0, 1), (1, 1)],
        }
    }
    fn color() -> Color {
        match rand::rng().random_range(0..9) {
            0 => Color::Cyan,
            1 => Color::Yellow,
            2 => Color::Magenta,
            3 => Color::LightYellow,
            4 => Color::Blue,
            5 => Color::Green,
            6 => Color::LightRed,
            7 => Color::LightGreen,
            8 => Color::LightBlue,
            _ => Color::Red,
            
        }
    }
    fn random() -> Self {
        let kind = match rand::rng().random_range(0..7) {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::L,
            4 => Tetromino::J,
            5 => Tetromino::S,
            _ => Tetromino::Z,
        };
        Piece::new(kind)
    }
}

fn render_GO(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, board: &Board, score: u32) -> io::Result<()> {
    terminal.draw(|f| {
        let size = f.area();
        let wb = (W*2+2) as u16;
        let hb = (H+2) as u16;
        let sw = 28;
        let vert = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Min(0),
            Constraint::Length(hb),
            Constraint::Min(0),
        ]).split(size);
        let horiz = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Min(0),
            Constraint::Length(wb),
            Constraint::Length(sw),
            Constraint::Min(0),
        ]).split(vert[1]);
        let ba = horiz[1];
        let ia = horiz[2];
        let mut lines = Vec::new();
        for y in 0..H {
            let mut spans = Vec::new();
            for x in 0..W {
                if let Some(color) = board.cells[y][x] {
                    spans.push(Span::styled("██", Style::default().fg(color)));
                } else {
                    spans.push(Span::raw("  "));
                }
            }
            lines.push(Line::from(spans));
        }
        let board_widget = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Tetris"));
        f.render_widget(board_widget, ba);
        let info = Paragraph::new(vec![
            Line::from(Span::styled("YOU FRICKIN LOSER!!", Style::default().fg(Color::Red))),
            Line::from(""),
            Line::from(format!("Score: {}", score)),
            Line::from(""),
            Line::from("Press any key"),
            Line::from("to exit..."),
        ]).block(Block::default().borders(Borders::ALL).title("Info"));
        f.render_widget(info, ia);
    })?;
    Ok(())
}

fn main() {
    let mut board = Board::new();
    let mut piece = Piece::random();
    let mut ml = false;
    let mut mr = false;
    let mut md = false;
    let mut exit = false;
    let mut game_over = false;
    let mut score: u32 = 0;
    let mut lock: Option<Instant> = None;
    let delay = Duration::from_millis(300);
    let mut is_ghost = true;
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();
    loop {
        //ghost piece logic
        let mut ghost = piece.clone();
        while !board.is_occ(&ghost, 0, 1) {
            ghost.y += 1;
        }
        
        board.render(&mut terminal, &piece, &ghost, score, is_ghost);
        
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind != KeyEventKind::Repeat && key_event.kind != KeyEventKind::Release {
                    match key_event.code {
                        // hard drop logic
                        KeyCode::Char(' ') => {
                            while !board.is_occ(&piece, 0, 1) {
                                piece.y += 1;
                            }
                            for (dx, dy) in piece.blocks {
                                let px = piece.x + dx;
                                let py = piece.y + dy;
                                board.set_cell(px as usize, py as usize, piece.color);
                            }
                            let cleared = board.clear_lines();
                            score += match cleared {
                                1 => 100,
                                2 => 300,
                                3 => 500,
                                4 => 800,
                                _ => 0,
                            };
                            piece = Piece::random();
                            lock = None;
                            if board.is_occ(&piece, 0, 0) {
                                for (dx, dy) in piece.blocks {
                                    let px = piece.x + dx;
                                    let py = piece.y + dy;
                                    if px >= 0 && py >= 0 && px < W as i32 && py < H as i32 {
                                        board.set_cell(px as usize, py as usize, piece.color);
                                    }
                                }
                                game_over = true;
                            }
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('w') | KeyCode::Char('W') => {
                            let mut rpiece = piece.clone();
                            rpiece.rotate();
                            if board.can_rot(&rpiece) {
                                piece = rpiece;
                            }
                        }
                        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => ml = true,
                        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => mr = true,
                        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => md = true,
                        KeyCode::Char('g') | KeyCode::Char('G') => is_ghost = !is_ghost,
                        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => exit = true,
                        KeyCode::Esc => exit = true,
                        _ => {},
                    }
                }
            }
        }
        
        if exit {
            break;
        }
        
        if game_over {
            render_GO(&mut terminal, &board, score).unwrap();
            loop {
                if let Event::Key(_) = event::read().unwrap() {
                    break;
                }
            }
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
            if lock.is_none() {
                lock = Some(Instant::now());
            }
            else if lock.unwrap().elapsed() >= delay {
                for (dx, dy) in piece.blocks {
                    let px = piece.x + dx;
                    let py = piece.y + dy;
                    if px >= 0 && py >= 0 && px < W as i32 && py < H as i32 {
                        board.set_cell(px as usize, py as usize, piece.color);
                    }
                }
                let cleared = board.clear_lines();
                score += match cleared {
                    1 => 100,
                    2 => 300,
                    3 => 500,
                    4 => 800,
                    _ => 0,
                };
                piece = Piece::random();
                lock = None;
                if board.is_occ(&piece, 0, 0) {
                    for (dx, dy) in piece.blocks {
                        let px = piece.x + dx;
                        let py = piece.y + dy;
                        if px >= 0 && py >= 0 && px < W as i32 && py < H as i32 {
                            board.set_cell(px as usize, py as usize, piece.color);
                        }
                    }
                    game_over = true;
                }
            }
        }
        else {
            piece.y += 1;
            lock = None;
        }
    }
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}