mod game;
mod score;

use crate::game::{Board, Movement};

#[cfg(feature = "terminal")]
use crossterm::event::{self, Event, KeyCode};
#[cfg(feature = "terminal")]
use ratatui::DefaultTerminal;

#[cfg(feature = "web")]
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "web")]
use ratzilla::{DomBackend, WebRenderer, event::KeyCode};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};

#[derive(Default)]
struct App {
    board: Board,
    should_quit: bool,
    game_over: bool,
}

enum AppEvent {
    GameOver,
    MoveBoard(Movement),
    RestartGame,
    Quit,
}

impl App {
    fn handle_event(&mut self, app_event: AppEvent) -> Option<AppEvent> {
        match app_event {
            AppEvent::GameOver => self.game_over = true,
            AppEvent::Quit => self.should_quit = true,
            AppEvent::RestartGame => {
                self.board = Board::default();
                self.game_over = false;
                let _ = self.board.spawn_random_cell();
            }
            AppEvent::MoveBoard(movement) => {
                let board_has_changed = self.board.move_board(movement);
                if board_has_changed {
                    self.board.update_score();
                    match self.board.spawn_random_cell() {
                        Ok(_) => (),
                        Err(_) => {
                            if !self.board.is_board_movable() {
                                return Some(AppEvent::GameOver);
                            }
                        }
                    }
                } else if !self.board.is_board_movable() {
                    return Some(AppEvent::GameOver);
                }
            }
        }
        None
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Min(1)])
            .split(frame.area());

        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(4), Constraint::Max(4), Constraint::Max(4)])
            .spacing(1)
            .split(layout[1]);

        frame.render_widget(&self.board, layout[0]);
        frame.render_widget(&self.board.score, sub_layout[0]);

        frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![
                    "Press ".into(),
                    "r".bold().yellow(),
                    " to reset the game".into(),
                ]),
                "or".bold().into(),
                Line::from(vec![
                    "Press ".into(),
                    "q".bold().yellow(),
                    " to quit".into(),
                ]),
            ]),
            sub_layout[1],
        );
        if self.game_over {
            frame.render_widget(
                Paragraph::new(vec!["GAME OVER".bold().red().into()]),
                sub_layout[2],
            );
        }
    }
}

// ── Terminal entry point ──────────────────────────────────────────────

#[cfg(feature = "terminal")]
fn main() -> color_eyre::Result<()> {
    let mut app = App::default();
    let _ = app.board.spawn_random_cell();
    color_eyre::install()?;
    ratatui::run(|t| app.run(t))?;
    Ok(())
}

#[cfg(feature = "terminal")]
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

    fn handle_key(&mut self, key: event::KeyEvent, _event: Event) -> Option<AppEvent> {
        match key.code {
            KeyCode::Char('q') => Some(AppEvent::Quit),
            KeyCode::Char('j') | KeyCode::Down => Some(AppEvent::MoveBoard(Movement::Down)),
            KeyCode::Char('k') | KeyCode::Up => Some(AppEvent::MoveBoard(Movement::Up)),
            KeyCode::Char('h') | KeyCode::Left => Some(AppEvent::MoveBoard(Movement::Left)),
            KeyCode::Char('l') | KeyCode::Right => Some(AppEvent::MoveBoard(Movement::Right)),
            KeyCode::Char('r') => Some(AppEvent::RestartGame),
            _ => None,
        }
    }
}

// ── Web entry point ───────────────────────────────────────────────────

#[cfg(feature = "web")]
fn main() -> std::io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = ratatui::Terminal::new(backend)?;

    let app = Rc::new(RefCell::new(App::default()));
    let _ = app.borrow_mut().board.spawn_random_cell();

    terminal.on_key_event({
        let app = app.clone();
        move |key_event| {
            let mut app = app.borrow_mut();
            let app_event = match key_event.code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                KeyCode::Char('j') | KeyCode::Down => Some(AppEvent::MoveBoard(Movement::Down)),
                KeyCode::Char('k') | KeyCode::Up => Some(AppEvent::MoveBoard(Movement::Up)),
                KeyCode::Char('h') | KeyCode::Left => Some(AppEvent::MoveBoard(Movement::Left)),
                KeyCode::Char('l') | KeyCode::Right => Some(AppEvent::MoveBoard(Movement::Right)),
                KeyCode::Char('r') => Some(AppEvent::RestartGame),
                _ => None,
            };

            if let Some(evt) = app_event {
                let mut next = app.handle_event(evt);
                while let Some(evt) = next {
                    next = app.handle_event(evt);
                }
            }
        }
    });

    terminal.draw_web({
        let app = app.clone();
        move |frame| {
            let app = app.borrow();
            app.render(frame);
        }
    });

    Ok(())
}

// ── Fallback if no feature is enabled ─────────────────────────────────

#[cfg(not(any(feature = "terminal", feature = "web")))]
fn main() {
    compile_error!("Either the 'terminal' or 'web' feature must be enabled");
}
