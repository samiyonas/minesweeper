use ratatui::{ self, Frame };
use ratatui::layout::{ Layout, Constraint };
use ratatui::style::{ Color, Style };
use ratatui::widgets::{Block, Paragraph};
use ratatui::prelude;
use crossterm::event::{ KeyCode };
use std::io;
use ratatui::DefaultTerminal;
mod render_logic;
mod board_state;
use render_logic::MineSweeperWidget;
use board_state::BoardState;

const WIDTH: u16 = 16;
const HEIGHT: u16 = 30;
const MINE_COUNT: u16 = 99;

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
