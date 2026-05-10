use ratatui::{ self, Frame };
use ratatui::layout::{ Layout, Constraint };
use ratatui::prelude::{self, Rect, Buffer, StatefulWidget};
use ratatui::style::{ Color, Style };
use ratatui::widgets::{Block, Paragraph};
use crossterm::event::{ KeyCode };
use std::io;
use ratatui::DefaultTerminal;
use rand::{ Rng };

const WIDTH: u16 = 16;
const HEIGHT: u16 = 30;
const MINE_COUNT: u16 = 99;

#[derive(Clone, PartialEq)]
struct Cell {
    is_mine: bool,
    neighbour_mines: u16,
    cell_state: CellState
}
impl Cell {
    fn default() -> Self {
        Self {
            is_mine: false,
            neighbour_mines: 0,
            cell_state: CellState::Hidden
        }
    }
}
#[derive(Clone, PartialEq)]
enum CellState {
    Hidden,
    Revealed,
    Neighbour
}
struct BoardState {
    cursor_x: u16,
    cursor_y: u16,
    width: u16,
    height: u16,
    grid: Vec<Vec<Cell>>,
    game_over: bool
}

impl BoardState {
    fn new(width: u16, height: u16, mine_count: u16) -> Self {
        let mut board = Self {
            width,
            height,
            cursor_x: 0,
            cursor_y:  0,
            grid: vec![vec![Cell::default(); height as usize]; width as usize],
            game_over: false
        };
        board.plant_mines(mine_count);
        board
    }

    fn plant_mines(&mut self, mine_count: u16) {
        let mut planted = 0;
        let mut rng = rand::thread_rng();
        while planted < mine_count {
            let x = rng.gen_range(0..self.width) as usize;
            let y = rng.gen_range(0..self.height) as usize;

            if !self.grid[x][y].is_mine {
                self.grid[x][y].is_mine = true;
                planted += 1;
            }
        }
    }

    fn graph(&mut self, x: i16, y: i16) {
        let directions: [(i16, i16); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];
        if self.game_over {
            return;
        }
        if self.grid[x as usize][y as usize].is_mine {
            self.game_over = true;
            return
        }

        if self.grid[x as usize][y as usize].cell_state != CellState::Hidden {
            return
        }

        let mut mines = 0;
        for (r, c) in directions {
            let nr = (r + x) as usize;
            let nc = (c + y) as usize;

            if nr < self.width as usize
                && nc < self.height as usize
                && self.grid[nr][nc].is_mine
            {
                mines += 1;
            }
        }
        if mines > 0 {
            self.grid[x as usize][y as usize].neighbour_mines = mines;
            self.grid[x as usize][y as usize].cell_state = CellState::Neighbour;
            return
        }

        self.grid[x as usize][y as usize].cell_state = CellState::Revealed;
        for (r, c) in directions {
            let nr = (r + x) as usize;
            let nc = (c + y) as usize;

            if nr < self.width as usize
                && nc < self.height as usize
                && self.grid[nr][nc].cell_state == CellState::Hidden
            {
                self.graph(nr as i16, nc as i16);
            }
        }
    }
    fn reveal_mines(&mut self, area: Rect, buf: &mut Buffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell_x = area.x + (x * 2); // Multiplied by 2 for "square" look
                let cell_y = area.y + y;

                if cell_x < area.right() && cell_y < area.bottom() {
                    let style = if x == self.cursor_x && y == self.cursor_y {
                        Style::default().bg(Color::Yellow).fg(Color::Black) // Highlight cursor
                    } else {
                        Style::default()
                    };

                    if self.game_over && self.grid[x as usize][y as usize].is_mine {
                        buf.set_string(cell_x, cell_y, "x", style);
                    }
                }
            }
        }
    }
}
struct App {
    exit: bool,
    board_state: BoardState
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(key) = crossterm::event::read()?.as_key_event() {
                match key.code {
                    KeyCode::Char('q') => self.quit(),
                    KeyCode::Char('r') => self.reload_game(),
                    KeyCode::Char('h') | KeyCode::Left => self.move_left(),
                    KeyCode::Char('j') | KeyCode::Down => self.move_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.move_up(),
                    KeyCode::Char('l') | KeyCode::Right => self.move_right(),
                    KeyCode::Enter => self.select(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn select(&mut self) {
        self.board_state.graph(self.board_state.cursor_x as i16, self.board_state.cursor_y as i16);
    }
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3)
        ]).split(area);

        let widget = MineSweeperWidget;
        frame.render_stateful_widget(widget, layout[0], &mut self.board_state);

        let text = if self.board_state.game_over {
            "BOOM! [Q] Quit | [R] Replay"
        } else {
            "Arrows to Move | Enter to Select | [Q] Quit | [R] Replay"
        };

        let sty = if self.board_state.game_over {
            Style::default().fg(Color::LightRed)
        } else {
            Style::default().fg(Color::Cyan)
        };

        let instruction = Paragraph::new(text)
            .style(sty)
            .alignment(prelude::Alignment::Left)
            .block(Block::bordered().border_type(ratatui::widgets::BorderType::Rounded));

        frame.render_widget(instruction, layout[1]);
    }

    fn quit(&mut self) {
        self.exit = true;
    }
    fn reload_game(&mut self) {
        *self = Self {
            exit: false,
            board_state: BoardState::new(WIDTH, HEIGHT, MINE_COUNT)
        }
    }
    fn move_left(&mut self) {
        if self.board_state.cursor_x == 0 {
            self.board_state.cursor_x = self.board_state.width - 1;
        } else {
            self.board_state.cursor_x -= 1;
        }
    }
    fn move_down(&mut self) {
        if self.board_state.cursor_y == self.board_state.height - 1 {
            self.board_state.cursor_y = 0;
        } else {
            self.board_state.cursor_y += 1;
        }
    }
    fn move_up(&mut self) {
        if self.board_state.cursor_y == 0 {
            self.board_state.cursor_y = self.board_state.height - 1;
        } else {
            self.board_state.cursor_y -= 1;
        }
    }
    fn move_right(&mut self) {
        if self.board_state.cursor_x == self.board_state.width - 1 {
            self.board_state.cursor_x = 0;
        } else {
            self.board_state.cursor_x += 1
        }
    }
}

struct MineSweeperWidget;
impl StatefulWidget for MineSweeperWidget {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        for y in 0..state.height {
            for x in 0..state.width {
                let cell_x = area.x + (x * 2); // Multiplied by 2 for "square" look
                let cell_y = area.y + y;

                if cell_x < area.right() && cell_y < area.bottom() {
                    let style = if x == state.cursor_x && y == state.cursor_y {
                        Style::default().bg(Color::Yellow).fg(Color::Black) // Highlight cursor
                    } else {
                        Style::default()
                    };

                    if state.game_over && state.grid[x as usize][y as usize].is_mine {
                        state.reveal_mines(area, buf);
                        continue;
                    }

                    match state.grid[x as usize][y as usize].cell_state {
                        CellState::Hidden => buf.set_string(cell_x, cell_y, "■ ", style),
                        CellState::Neighbour => buf.set_string(cell_x, cell_y, state.grid[x as usize][y as usize].neighbour_mines.to_string(), style),
                        CellState::Revealed => buf.set_string(cell_x, cell_y, ".", style)
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        board_state: BoardState::new(WIDTH, HEIGHT, MINE_COUNT)
    };
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}
