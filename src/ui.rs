use crate::app::{Appstate, Panel};
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Direction;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::{Line, ToSpan};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, BorderType, List, ListItem, Padding, Paragraph};

pub fn render(frame: &mut Frame, app_state: &mut Appstate) {
    if app_state.is_add_new {
        render_input_from(frame, app_state);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(frame.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[0]);

        render_list(frame, main_chunks[0], app_state);
        render_list_list(frame, main_chunks[1], app_state);

        let help_bar = Paragraph::new(" [q] quit | [i] new | [s] sub | [j/k] nav | [E] edit | [d] del | [J/K] move | [gg/G] top/bottom ")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Rgb(166, 227, 161)));
        frame.render_widget(help_bar, chunks[1]);
    }
}

pub fn render_input_from(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let line = Line::from(app_state.input_value.as_str());
    let input_paragraph = Paragraph::new(line);
    let border_area = frame.area();
    let chuncks = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area);

    if chuncks.is_empty() {
        let error_message = Paragraph::new("No area available to render input block");
        frame.render_widget(error_message, frame.area());
        return;
    }

    let inner_area = chuncks[0];

    let input_block = Block::bordered()
        .title(" Input description ".to_span().into_centered_line())
        .fg(Color::Rgb(116, 199, 236))
        .border_type(BorderType::Rounded);
    frame.render_widget(input_block, border_area);
    frame.render_widget(input_paragraph, inner_area);
    frame.set_cursor((app_state.cursor_position + 1) as u16, 1);
}

pub fn render_list(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let inner_area = area;

    let list_index = if app_state.active_panel == Panel::NewList {
        app_state
            .lists_list_state
            .selected()
            .unwrap_or(app_state.current_list_index)
    } else {
        app_state.current_list_index
    };

    let current_list = app_state.lists.get(list_index);
    let list_name = match current_list {
        Some(list) => list.name.clone(),
        None => "Empty".to_string(),
    };

    let title_str = format!(" {} ", list_name);

    let mut items: Vec<ListItem> = match current_list {
        Some(list) => {
            let mut result = Vec::new();
            for x in &list.items {
                let count_str = if x.sub_items.is_empty() {
                    String::new()
                } else {
                    let completed = x.sub_items.iter().filter(|s| s.is_done).count();
                    let total = x.sub_items.len();
                    format!(" ({}/{})", completed, total)
                };
                let prefix = if x.is_done { "[x]" } else { "[ ]" };
                let full_text = format!("{} {}{}", prefix, x.description, count_str);
                let line = Line::from(full_text);
                result.push(ListItem::new(line));
                for sub in &x.sub_items {
                    let sub_prefix = if sub.is_done { "[x]" } else { "[ ]" };
                    let sub_text = format!("  {} {}", sub_prefix, sub.description);
                    let sub_line = Line::from(sub_text);
                    result.push(ListItem::new(sub_line));
                }
            }
            result
        }
        None => Vec::new(),
    };

    let list = List::new(items)
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Green))
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(0))
                .title(title_str.to_span().into_centered_line())
                .fg(if app_state.active_panel == Panel::List {
                    Color::Rgb(203, 166, 247)
                } else {
                    Color::DarkGray
                })
        );
    frame.render_stateful_widget(list, area, &mut app_state.list_state);
}

pub fn render_new_list(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let title = " New List ".to_span();

    Paragraph::new(app_state.input_value.as_str())
        .block(
            Block::bordered()
                .title(title)
                .fg(Color::Rgb(203, 166, 247))
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1)),
        )
        .render(area, frame.buffer_mut());
}

pub fn render_list_list(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let list_items: Vec<ListItem> = app_state
        .lists
        .iter()
        .enumerate()
        .map(|(i, list)| {
            let name = if i == app_state.current_list_index {
                format!("{}•", list.name)
            } else {
                list.name.clone()
            };
            ListItem::from(name)
        })
        .collect();
    let list =
        List::new(list_items)
            .highlight_symbol(">")
            .block(Block::bordered().title(" Lists ").fg(
                if app_state.active_panel == Panel::NewList {
                    Color::Rgb(203, 166, 247)
                } else {
                    Color::DarkGray
                },
            ));
    frame.render_stateful_widget(list, area, &mut app_state.lists_list_state);
}
