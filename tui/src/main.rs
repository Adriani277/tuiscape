use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::ListState,
    DefaultTerminal,
};
use strum::IntoEnumIterator;
use tuiscape_core::player::Player;
use tuiscape_core::{
    domain::{
        level_data::{Level, Xp},
        skill_method::{SkillMethod, SkillMethodData},
        skills::skill_type::Skill,
    },
    storage,
};

mod tui;

// ── Runtime ──────────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    color_eyre::install()?;

    let (sender, receiver) = mpsc::channel::<Messages>();
    let event_sender = sender.clone();
    let terminal = ratatui::init();

    // Tick thread
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            thread::sleep(Duration::from_millis(100));
            let delta = last_tick.elapsed();
            last_tick = Instant::now();
            if sender.send(Messages::Tick(delta)).is_err() {
                break;
            }
        }
    });

    // Key event thread
    thread::spawn(move || loop {
        if let Ok(Event::Key(key)) = event::read() {
            let message = handle_key(key);
            if event_sender.send(message).is_err() {
                break;
            }
        }
    });

    let result = Model::default().run(terminal, receiver);

    ratatui::restore();
    result
}

// ── Model ────────────────────────────────────────────────────────────────────

struct Model {
    should_exit: bool,
    skill_progress: Duration,
    view: View,
    active_skill: Option<SkillMethod>,
    player: Player,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            should_exit: false,
            skill_progress: Duration::ZERO,
            view: View::Skills(ListState::default()),
            active_skill: None,
            player: Player::init(),
        }
    }
}

impl Model {
    fn run(mut self, mut terminal: DefaultTerminal, receiver: Receiver<Messages>) -> Result<()> {
        while !self.should_exit {
            let message = receiver.recv()?;
            update(&mut self, message);
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
        }
        Ok(())
    }
}

// ── View ─────────────────────────────────────────────────────────────────────

enum View {
    Skills(ListState),
    Methods {
        skill_type: Skill,
        skill_state: ListState,
        method_state: ListState,
    },
}

// ── Messages ─────────────────────────────────────────────────────────────────

enum ListAction {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
}

enum Messages {
    Quit,
    Navigate(ListAction),
    Confirm,
    Tick(Duration),
    Noop,
    StopSkilling,
    NavigateBack,
}

// ── Update ───────────────────────────────────────────────────────────────────

fn update(model: &mut Model, message: Messages) {
    match message {
        Messages::Quit => {
            storage::store_player_data(&model.player);
            model.should_exit = true
        }
        Messages::Navigate(list_action) => {
            let state = match &mut model.view {
                View::Skills(list_state) => list_state,
                View::Methods {
                    skill_type: _,
                    skill_state: _,
                    method_state,
                } => method_state,
            };
            list_navigation(state, list_action);
        }

        Messages::Confirm => match &mut model.view {
            View::Methods {
                skill_type,
                skill_state: _,
                method_state,
            } => {
                if let Some(idx) = method_state.selected() {
                    model.skill_progress = Duration::ZERO;
                    model.active_skill = skill_type.methods().into_iter().nth(idx);
                }
            }
            View::Skills(list_state) => {
                let selected_index = list_state.selected();
                if let Some(index) = selected_index {
                    let skill_op = Skill::iter().nth(index);
                    if let Some(skill) = skill_op {
                        model.view = View::Methods {
                            skill_type: skill,
                            skill_state: list_state.clone(),
                            method_state: ListState::default(),
                        };
                    }
                }
            }
        },

        Messages::StopSkilling => {
            model.skill_progress = Duration::ZERO;
            model.active_skill = None
        }

        Messages::Tick(tick_duration) => {
            if let Some(active) = &model.active_skill {
                let accum = model
                    .player
                    .skill_tick(active, tick_duration, model.skill_progress);

                model.skill_progress = accum;
            }
        }

        Messages::NavigateBack => match &mut model.view {
            View::Skills(_) => (),
            View::Methods {
                skill_type: _,
                skill_state,
                method_state: _,
            } => model.view = View::Skills(skill_state.clone()),
        },

        Messages::Noop => {}
    }
}

fn list_navigation(list_state: &mut ListState, list_action: ListAction) {
    match list_action {
        ListAction::MoveUp => list_state.select_previous(),
        ListAction::MoveDown => list_state.select_next(),
        ListAction::MoveTop => list_state.select_first(),
        ListAction::MoveBottom => list_state.select_last(),
    }
}

fn handle_key(key: event::KeyEvent) -> Messages {
    if key.kind != KeyEventKind::Press {
        return Messages::Noop;
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => Messages::Navigate(ListAction::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Messages::Navigate(ListAction::MoveUp),
        KeyCode::Char('g') | KeyCode::Home => Messages::Navigate(ListAction::MoveTop),
        KeyCode::Char('G') | KeyCode::End => Messages::Navigate(ListAction::MoveBottom),
        KeyCode::Char('q') => Messages::Quit,
        KeyCode::Char('s') | KeyCode::Char('S') => Messages::StopSkilling,
        KeyCode::Enter => Messages::Confirm,
        KeyCode::Esc => Messages::NavigateBack,
        _ => Messages::Noop,
    }
}
