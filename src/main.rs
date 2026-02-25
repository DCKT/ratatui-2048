mod game;
mod score;

use crate::game::{Board, Movement};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};

#[derive(Default)]
struct App {
    board: Board,
    should_quit: bool,
    game_over: bool,
}

fn main() -> color_eyre::Result<()> {
    let mut app = App::default();
    color_eyre::install()?;
    ratatui::run(|t| app.run(t))?;
    Ok(())
}

enum AppEvent {
    GameOver,
    MoveBoard(Movement),
    Quit,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            let event = crossterm::event::read()?;

            if let crossterm::event::Event::Key(key) = event {
                let mut app_event = self.handle_key(key, event);

                while app_event.is_some() {
                    app_event = self.handle_event(app_event.unwrap());
                }
            }
        }
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Min(1)])
            .split(frame.area());

        frame.render_widget(&self.board, layout[0]);
        frame.render_widget(&self.board.score, layout[1]);
    }

    fn handle_key(&mut self, key: event::KeyEvent, _event: Event) -> Option<AppEvent> {
        match key.code {
            KeyCode::Char('q') => Some(AppEvent::Quit),
            KeyCode::Char('j') | KeyCode::Down => Some(AppEvent::MoveBoard(Movement::Down)),
            KeyCode::Char('k') | KeyCode::Up => Some(AppEvent::MoveBoard(Movement::Up)),
            KeyCode::Char('h') | KeyCode::Left => Some(AppEvent::MoveBoard(Movement::Left)),
            KeyCode::Char('l') | KeyCode::Right => Some(AppEvent::MoveBoard(Movement::Right)),
            _ => None,
        }
    }

    fn handle_event(&mut self, app_event: AppEvent) -> Option<AppEvent> {
        match app_event {
            AppEvent::GameOver => self.game_over = true,
            AppEvent::Quit => self.should_quit = true,
            AppEvent::MoveBoard(movement) => {
                let board_has_changed = self.board.move_board(movement);
                if board_has_changed {
                    self.board.update_score();
                    match self.board.spawn_random_cell() {
                        Ok(_) => (),
                        Err(_) => {
                            // check if board is movable; else game over
                            let is_board_movable = self.board.is_board_movable();

                            if !is_board_movable {
                                return Some(AppEvent::GameOver);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
