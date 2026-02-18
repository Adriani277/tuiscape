#!/usr/bin/env rust-script

use core::time;
use std::thread::sleep;

use clap::Parser;
use color_eyre::Result;
use strum::IntoEnumIterator;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{palette::tailwind::SLATE, Color},
    widgets::{List, ListItem, ListState, Widget},
    DefaultTerminal, Frame,
};

const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the skill
    #[arg(short, long)]
    name: String,
}

#[derive(Debug, PartialEq)]
enum Focus {
    Skills,
    Methods,
}

struct App {
    should_exit: bool,
    skills_list: SkillsList,
    skill_progress: u16,
    focus: Focus,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            skills_list: SkillsList {
                skills: SkillType::iter().collect(),
                state: ListState::default(),
            },
            skill_progress: 0,
            focus: Focus::Skills,
        }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        let list = match self.focus {
            Focus::Skills => &mut self.skills_list.state,
            Focus::Methods => todo!(),
        };

        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Enter => match self.focus {
                Focus::Skills => self.focus = Focus::Methods,
                Focus::Methods => self.focus = Focus::Skills,
            },
            _ => {}
        }

        match key.code {
            KeyCode::Char('h') | KeyCode::Left => list.select(None),
            KeyCode::Char('j') | KeyCode::Down => list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => list.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => list.select_first(),
            KeyCode::Char('G') | KeyCode::End => list.select_last(),
            _ => {}
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_gauge(item_area, buf);
    }
}

struct SkillsList {
    skills: Vec<SkillType>,
    state: ListState,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();
    result

    // let args = Args::parse();

    // println!("Starting {}", args.name);

    // let skill_type = SkillType::from_str(args.name.as_str()).unwrap_or_else(|_| {
    //     let all: Vec<_> = SkillType::iter().collect();
    //     eprintln!("Skill must be one of: {:?}", all);
    //     std::process::exit(1);
    // });

    // let skill = Skill::new(skill_type);

    // print_skill(skill);
}

fn app(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let items: Vec<ListItem> = SkillType::iter()
        .enumerate()
        .map(|(_, it)| ListItem::from(format!("{:?}", it)))
        .collect();

    let lst = List::new(items);
    frame.render_widget(lst, frame.area());
}

#[derive(Debug, strum::EnumIter, strum::EnumString, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "lowercase")]
enum SkillType {
    Fishing,
    Firemaking,
    WoodCutting,
    Cooking,
}

impl From<&SkillType> for ListItem<'_> {
    fn from(value: &SkillType) -> Self {
        ListItem::new(format!("{:?}", value))
    }
}

fn print_skill(mut skill: Skill) {
    let xp_amount = 10 as u32;
    Skill::add_xp(&mut skill, xp_amount);
    println!("Gained {} in {:?}", &xp_amount, &skill);
    sleep(time::Duration::from_millis(1000));
}

#[derive(Debug)]
struct Skill {
    skill_type: SkillType,
    level: u8,
    xp: u32,
    xp_to_next_level: u32,
}

impl Skill {
    fn new(skill_type: SkillType) -> Self {
        Skill {
            skill_type,
            xp: 0,
            xp_to_next_level: 100,
            level: 1,
        }
    }

    fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        if self.xp >= self.xp_to_next_level {
            self.level += 1;
            self.xp_to_next_level += 100
        }
    }
}

mod tui {
    use ratatui::{
        prelude::{Buffer, Rect},
        style::{Style, Stylize},
        symbols,
        text::Line,
        widgets::{
            Block, Borders, Gauge, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget,
            Widget,
        },
    };

    use crate::Focus;

    use super::alternate_colors;

    use super::App;

    /// Rendering logic
    impl App {
        pub(crate) fn render_header(area: Rect, buf: &mut Buffer) {
            Paragraph::new("T U I S C A P E")
                .bold()
                .centered()
                .green()
                .render(area, buf);
        }

        pub(crate) fn render_footer(area: Rect, buf: &mut Buffer) {
            Paragraph::new(
                "Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.",
            )
            .centered()
            .render(area, buf);
        }

        pub(crate) fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
            let (title, items, state) = match self.focus {
                Focus::Skills => {
                    let items: Vec<ListItem> = self
                        .skills_list
                        .skills
                        .iter()
                        .enumerate()
                        .map(|(i, skill)| ListItem::from(skill).bg(alternate_colors(i)))
                        .collect();
                    ("Skills", items, &mut self.skills_list.state)
                }
                Focus::Methods => {
                    todo!()
                }
            };

            let block = Block::new()
                .title(Line::raw(title).centered())
                .borders(Borders::all())
                .border_set(symbols::border::ROUNDED)
                .border_style(Style::new().blue());

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::new().red())
                .highlight_symbol("->")
                .highlight_spacing(HighlightSpacing::Always);

            StatefulWidget::render(list, area, buf, state);
        }

        pub(crate) fn render_gauge(&mut self, area: Rect, buf: &mut Buffer) {
            let title = Block::new()
                .title("Progress")
                .borders(Borders::all())
                .border_style(Style::new().green())
                .border_set(symbols::border::ROUNDED);

            self.skill_progress += 1;

            Gauge::default()
                .block(title)
                .blue()
                .gauge_style(Style::new().blue())
                .percent(self.skill_progress.clamp(0, 100))
                .render(area, buf);
        }
        // fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        //     Paragraph::new("TEST").render(area, buf);
        // }
    }
}
