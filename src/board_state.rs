use rand::{ Rng };
use ratatui::style::{ Color, Style };
use ratatui::prelude::{ Rect, Buffer };

#[derive(Clone, PartialEq)]
pub struct Cell {
    pub is_mine: bool,
    pub neighbour_mines: u16,
    pub cell_state: CellState
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
pub enum CellState {
    Hidden,
    Revealed,
    Neighbour
}

pub struct BoardState {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub width: u16,
    pub height: u16,
    pub grid: Vec<Vec<Cell>>,
    pub game_over: bool
}

impl BoardState {
    pub fn new(width: u16, height: u16, mine_count: u16) -> Self {
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

    pub fn plant_mines(&mut self, mine_count: u16) {
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

    pub fn graph(&mut self, x: i16, y: i16) {
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

    pub fn reveal_mines(&mut self, area: Rect, buf: &mut Buffer) {
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
