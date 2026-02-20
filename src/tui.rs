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

use crate::alternate_colors;

use crate::App;

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
