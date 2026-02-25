use rand::seq::IndexedRandom;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph, Widget},
};

use crate::score::Score;

type Cell = Option<i32>;

type Row = [Cell; 4];

pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone)]
pub struct Board {
    state: [Row; 4],
    pub score: Score,
}

#[derive(Clone)]
pub enum Movement {
    Up,
    Down,
    Right,
    Left,
}

impl Board {
    pub fn set_cell(&mut self, cell_value: Cell, x: usize, y: usize) {
        self.state[x][y] = cell_value;
    }

    fn get_score(&self) -> i32 {
        self.state.iter().flatten().flatten().sum()
    }

    pub fn update_score(&mut self) {
        self.score = Score {
            value: self.get_score(),
        };
    }

    fn get_empty_cells(&self) -> Vec<Coordinates> {
        let mut empty_cells: Vec<Coordinates> = vec![];

        for x in 0..4 {
            for y in 0..4 {
                if self.state[x][y].is_none() {
                    empty_cells.push(Coordinates { x, y });
                }
            }
        }

        empty_cells
    }

    pub fn spawn_random_cell(&mut self) -> Result<(), ()> {
        if let Some(random_cell) = self.get_empty_cells().choose(&mut rand::rng()) {
            let new_cell_value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
            self.state[random_cell.x][random_cell.y] = Some(new_cell_value);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn is_board_movable(&mut self) -> bool {
        let mut board = self.clone();

        [
            Movement::Up,
            Movement::Left,
            Movement::Down,
            Movement::Right,
        ]
        .iter()
        .any(|m| board.move_board(m.clone()))
    }

    pub fn move_board(&mut self, movement: Movement) -> bool {
        let mut has_changed = false;

        for col_index in 0..4 {
            let (mut destination_cursor, mut iterator_index): (usize, isize) = match movement {
                Movement::Up | Movement::Left => (0, 0),
                Movement::Down | Movement::Right => (3, 3),
            };

            while match movement {
                Movement::Up | Movement::Left => iterator_index < 4,
                Movement::Down | Movement::Right => iterator_index >= 0,
            } {
                if destination_cursor as isize != iterator_index {
                    let (current_x, current_y) = match movement {
                        Movement::Up | Movement::Down => (destination_cursor, col_index),
                        Movement::Left | Movement::Right => (col_index, destination_cursor),
                    };
                    let (next_x, next_y) = match movement {
                        Movement::Up | Movement::Down => (iterator_index as usize, col_index),
                        Movement::Left | Movement::Right => (col_index, iterator_index as usize),
                    };

                    match (self.state[current_x][current_y], self.state[next_x][next_y]) {
                        (None, Some(o)) => {
                            self.set_cell(Some(o), current_x, current_y);
                            self.set_cell(None, next_x, next_y);
                            has_changed = true;
                        }
                        (Some(d), Some(o)) => {
                            if d == o {
                                self.set_cell(Some(d + o), current_x, current_y);
                                self.set_cell(None, next_x, next_y);
                                has_changed = true;

                                match movement {
                                    Movement::Up | Movement::Left => destination_cursor += 1,
                                    Movement::Down | Movement::Right => destination_cursor -= 1,
                                };
                            } else {
                                match movement {
                                    Movement::Up | Movement::Left => destination_cursor += 1,
                                    Movement::Down | Movement::Right => destination_cursor -= 1,
                                };
                                continue;
                            }
                        }
                        _ => (),
                    }
                }
                match movement {
                    Movement::Up | Movement::Left => iterator_index += 1,
                    Movement::Down | Movement::Right => iterator_index -= 1,
                };
            }
        }

        has_changed
    }
}

impl Widget for &Board {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let col_constraints = (0..4).map(|_| Constraint::Length(8));
        let row_constraints = (0..4).map(|_| Constraint::Length(4));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let x = i / 4;
            let y = i % 4;
            Paragraph::new(
                self.state[x][y]
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string()),
            )
            .block(Block::bordered())
            .render(cell, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_board() {
        let mut board = Board::default();
        board.set_cell(Some(1), 1, 0);

        assert_eq!(board.state[1][0], Some(1));
    }

    #[test]
    fn tets_score() {
        let mut board = Board::default();
        assert_eq!(board.get_score(), 0);

        board.set_cell(Some(2), 0, 0);
        assert_eq!(board.get_score(), 2);
        board.set_cell(Some(2), 0, 1);
        assert_eq!(board.get_score(), 4);
    }

    #[test]
    fn test_board_move_down() {
        let mut board = Board::default();
        board.set_cell(Some(1), 1, 0);
        assert_eq!(board.state[1][0], Some(1));
        board.move_board(Movement::Down);

        assert_eq!(board.state[0][0], None);
        assert_eq!(board.state[1][0], None);
        assert_eq!(board.state[2][0], None);
        assert_eq!(board.state[3][0], Some(1));

        let mut board2 = Board::default();
        board2.set_cell(Some(2), 0, 0);
        board2.set_cell(Some(2), 1, 0);
        board2.set_cell(Some(2), 2, 0);
        board2.set_cell(Some(2), 3, 0);
        assert_eq!(board2.state[1][0], Some(2));
        board2.move_board(Movement::Down);
        assert_eq!(board2.state[0][0], None, "first row should be empty");
        assert_eq!(board2.state[1][0], None, "second row should be empty");
        assert_eq!(board2.state[2][0], Some(4), "third row should equal 4");
        assert_eq!(board2.state[3][0], Some(4), "fourth row should equal 4");

        let mut board3 = Board::default();
        board3.set_cell(None, 0, 0);
        board3.set_cell(Some(2), 1, 0);
        board3.set_cell(Some(4), 2, 0);
        board3.set_cell(Some(6), 3, 0);
        board3.move_board(Movement::Up);
        assert_eq!(board3.state[0][0], Some(2), "first row should equal 2");
        assert_eq!(board3.state[1][0], Some(4), "second row should equal 4");
        assert_eq!(board3.state[2][0], Some(6), "third row should equal 6");
        assert_eq!(board3.state[3][0], None, "fourth row should be empty");
    }

    #[test]
    fn test_board_move_up() {
        let mut board = Board::default();
        board.set_cell(Some(2), 0, 0);
        board.set_cell(Some(2), 1, 0);
        board.set_cell(Some(2), 2, 0);
        board.set_cell(Some(2), 3, 0);
        assert_eq!(board.state[1][0], Some(2));
        board.move_board(Movement::Up);
        assert_eq!(board.state[0][0], Some(4), "first row should equal 4");
        assert_eq!(board.state[1][0], Some(4), "second row should equal 4");
        assert_eq!(board.state[2][0], None, "third row should be empty");
        assert_eq!(board.state[3][0], None, "fourth row should be empty");

        // Impossible
        let mut board3 = Board::default();
        board3.set_cell(None, 0, 0);
        board3.set_cell(Some(2), 1, 0);
        board3.set_cell(Some(4), 2, 0);
        board3.set_cell(None, 3, 0);
        board3.move_board(Movement::Up);
        assert_eq!(board3.state[0][0], Some(2), "first row should equal 2");
        assert_eq!(board3.state[1][0], Some(4), "second row should equal 4");
        assert_eq!(board3.state[2][0], None, "third row should be empty");
        assert_eq!(board3.state[3][0], None, "fourth row should be empty");
    }

    #[test]
    fn test_board_move_right() {
        let mut board = Board::default();
        board.set_cell(Some(2), 0, 0);
        board.set_cell(Some(2), 0, 1);
        board.set_cell(Some(2), 0, 2);
        board.set_cell(Some(2), 0, 3);
        assert_eq!(board.state[0][1], Some(2));
        board.move_board(Movement::Right);
        assert_eq!(board.state[0][0], None, "first column should be empty");
        assert_eq!(board.state[0][1], None, "second column should be empty");
        assert_eq!(board.state[0][2], Some(4), "third column should equal 4");
        assert_eq!(board.state[0][3], Some(4), "fourth column should equal 4");

        // Impossible
        let mut board3 = Board::default();
        board3.set_cell(None, 0, 0);
        board3.set_cell(Some(2), 0, 1);
        board3.set_cell(Some(4), 0, 2);
        board3.set_cell(None, 0, 3);
        board3.move_board(Movement::Right);
        assert_eq!(board3.state[0][0], None, "first column should be empty");
        assert_eq!(board3.state[0][1], None, "second column should be empty");
        assert_eq!(board3.state[0][2], Some(2), "third column should equal 2");
        assert_eq!(board3.state[0][3], Some(4), "fourth column should equal 4");
    }

    #[test]
    fn test_board_move_left() {
        let mut board = Board::default();
        board.set_cell(Some(2), 0, 0);
        board.set_cell(Some(2), 0, 1);
        board.set_cell(Some(2), 0, 2);
        board.set_cell(Some(2), 0, 3);
        assert_eq!(board.state[0][1], Some(2));
        board.move_board(Movement::Left);
        assert_eq!(board.state[0][0], Some(4), "first column should equal 4");
        assert_eq!(board.state[0][1], Some(4), "second column should equal 4");
        assert_eq!(board.state[0][2], None, "third column should be empty");
        assert_eq!(board.state[0][3], None, "fourth column should be empty");

        // Impossible
        let mut board3 = Board::default();
        board3.set_cell(None, 0, 0);
        board3.set_cell(Some(2), 0, 1);
        board3.set_cell(Some(4), 0, 2);
        board3.set_cell(None, 0, 3);
        board3.move_board(Movement::Left);
        assert_eq!(board3.state[0][0], Some(2), "first column should equal 2");
        assert_eq!(board3.state[0][1], Some(4), "second column should equal 4");
        assert_eq!(board3.state[0][2], None, "third column should be empty");
        assert_eq!(board3.state[0][3], None, "fourth column should be empty");
    }

    #[test]
    fn test_board_is_movable() {
        let mut board = Board::default();
        for x in 0..4 {
            for y in 0..4 {
                board.set_cell(Some(x + y), x as usize, y as usize);
            }
        }
        assert!(
            !board.is_board_movable(),
            "board is full and can't be updated"
        );
        for x in 0..4 {
            for y in 0..4 {
                board.set_cell(Some(2), x as usize, y as usize);
            }
        }
        assert!(board.is_board_movable(), "board is full but can be updated");
    }
}
