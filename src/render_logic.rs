use ratatui::prelude::{ Rect, Buffer, StatefulWidget};
use crate::board_state::{BoardState, CellState};
use ratatui::style::{ Color, Style };

pub struct MineSweeperWidget;
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
