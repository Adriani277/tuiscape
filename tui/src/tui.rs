use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{palette::tailwind::SLATE, Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, Gauge, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget,
    },
};
use strum::IntoEnumIterator;
use tui_big_text::{BigText, PixelSize};
use tuiscape_core::domain::{
    level_data::xp_for_level, skill_method::SkillMethodData, skills::skill_type::Skill,
};

use crate::{Model, View};

const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 15;

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl Widget for &mut Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            let msg = format!(
                "Terminal too small ({cols}x{rows}). Minimum: {MIN_WIDTH}x{MIN_HEIGHT}.",
                cols = area.width,
                rows = area.height,
            );
            Paragraph::new(msg)
                .centered()
                .alignment(Alignment::Center)
                .render(area, buf);
            return;
        }

        let [s, header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Max(2),
            Constraint::Max(4),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .areas(area);

        let msg = format!("{}x{}", area.width, area.height);
        Paragraph::new(msg).right_aligned().render(s, buf);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Min(3), Constraint::Max(3)]).areas(main_area);

        let [nav_area, _, player_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .areas(list_area);

        Model::render_header(header_area, area, buf);
        Model::render_footer(footer_area, buf);
        self.render_list(nav_area, buf);
        self.render_player(player_area, buf);
        self.render_gauge(item_area, buf);
    }
}

impl Model {
    pub(crate) fn render_header(area: Rect, total: Rect, buf: &mut Buffer) {
        let mut binding = BigText::builder();
        let big_text = binding
            .pixel_size(PixelSize::HalfHeight)
            .centered()
            .lines(vec!["TUISCAPE".red().into()]);
        if total.height >= 20 && total.width >= 72 {
            let built = big_text.pixel_size(PixelSize::HalfHeight).build();
            built.render(area, buf);
        } else {
            big_text
                .pixel_size(PixelSize::Quadrant)
                .build()
                .render(area, buf);
        }
    }

    pub(crate) fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new(
                "Use ↓↑ to move, h to deselect, Enter to open,\ng/G to go top/bottom, s/S to stop skilling, q to quit.",
            )
            .centered()
            .render(area, buf);
    }

    pub(crate) fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let (title, items, state) = match &mut self.view {
            View::Skills(list_state) => {
                let items: Vec<ListItem> = Skill::iter()
                    .enumerate()
                    .map(|(i, skill)| ListItem::new(format!("{}", skill)).bg(alternate_colors(i)))
                    .collect();
                if list_state.selected().is_none() {
                    list_state.select_first()
                }
                ("Skills", items, list_state)
            }
            View::Methods {
                skill_type,
                skill_state: _,
                method_state,
            } => {
                let items: Vec<ListItem> = skill_type
                    .methods()
                    .into_iter()
                    .enumerate()
                    .map(|(i, m)| {
                        ListItem::new(vec![
                            Line::from(format!("{}", m)),
                            Line::from(format!(
                                "  {} Skill XP 🕐{}s",
                                m.xp_award_amount().0,
                                m.xp_award_duration().as_secs_f32()
                            )),
                        ])
                        .bg(alternate_colors(i))
                    })
                    .collect();
                if method_state.selected().is_none() {
                    method_state.select_first()
                }
                ("Methods", items, method_state)
            }
        };

        let block = Block::new()
            .title(Line::raw(title).centered())
            .borders(Borders::all())
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::new().cyan());

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().red())
            .highlight_symbol("->")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }

    pub(crate) fn render_player(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title("Skill Levels")
            .title_alignment(Alignment::Center)
            .borders(Borders::all())
            .border_style(Style::new().dark_gray())
            .border_set(symbols::border::ROUNDED);

        let txt = Skill::iter()
            .map(|skill| {
                let level = &self.player.level_data(&skill).level;
                format!("{}:{}/99", skill, level.0)
            })
            .collect::<Vec<String>>()
            .join("\n");
        Paragraph::new(txt).block(block).render(area, buf);
    }

    pub(crate) fn render_gauge(&self, area: Rect, buf: &mut Buffer) {
        if let Some(method) = &self.active_skill {
            let skill = method.skill_type();
            let level_data = self.player.level_data(&skill);
            let xp_for_next_level = xp_for_level(level_data.level + 1);
            let name = format!(
                " {} - {} | {}/{}",
                skill, method, level_data.xp.0, xp_for_next_level.0
            );

            let progress = (self.skill_progress.as_secs_f32()
                / method.xp_award_duration().as_secs_f32())
                * 100.0;

            let title = Block::new()
                .title(name)
                .borders(Borders::all())
                .border_style(Style::new().green())
                .border_set(symbols::border::ROUNDED);

            Gauge::default()
                .block(title)
                .blue()
                .gauge_style(Style::new().blue())
                .percent(progress as u16)
                .render(area, buf);
        }
    }
}
