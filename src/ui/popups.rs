use super::centered_rect;
use crate::app::state::{Appstate, EditTarget};
use crate::theme::Theme;
use ratatui::prelude::{Alignment, Stylize};
use ratatui::style::Style;
use ratatui::text::{Line, Span, ToSpan};
use ratatui::widgets::{Block, BorderType, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_input_from(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let area = centered_rect(60, 25, frame.area());
    let theme = &app_state.theme;

    let title = match app_state.edit_target {
        EditTarget::Item => "Enter description",
        EditTarget::ListName => "Enter list name",
        EditTarget::None => "Input",
    };
    let title_str = format!(" {} ", title);

    let input_paragraph = Paragraph::new(app_state.input_value.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.foreground))
        .block(
            Block::bordered()
                .title(title_str.to_span().into_centered_line())
                .fg(theme.accent2)
                .border_type(BorderType::Rounded),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(input_paragraph, area);

    let input_width = area.width.saturating_sub(2) as usize;
    if input_width > 0 {
        let cursor_x = area.x + 1 + (app_state.cursor_position % input_width) as u16;
        let cursor_y = area.y + 1 + (app_state.cursor_position / input_width) as u16;
        frame.set_cursor_position((cursor_x, cursor_y));
    } else {
        frame.set_cursor_position((area.x + 1, area.y + 1));
    }
}

pub fn render_search_input(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let area = centered_rect(60, 25, frame.area());
    let theme = &app_state.theme;

    let input_paragraph = Paragraph::new(app_state.search_input_value.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(theme.foreground))
        .block(
            Block::bordered()
                .title(" Search ".to_span().into_centered_line())
                .fg(theme.accent2)
                .border_type(BorderType::Rounded),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(input_paragraph, area);

    let input_width = area.width.saturating_sub(2) as usize;
    if input_width > 0 {
        let cursor_x = area.x + 1 + (app_state.search_cursor_position % input_width) as u16;
        let cursor_y = area.y + 1 + (app_state.search_cursor_position / input_width) as u16;
        frame.set_cursor_position((cursor_x, cursor_y));
    } else {
        frame.set_cursor_position((area.x + 1, area.y + 1));
    }
}

pub fn render_delete_confirmation(frame: &mut Frame<'_>, theme: &Theme) {
    let area = centered_rect(30, 20, frame.area());
    let paragraph = Paragraph::new("Delete? (y/n)")
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground))
        .block(
            Block::bordered()
                .title(" Confirmation ")
                .border_type(BorderType::Rounded)
                .fg(theme.red),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

pub fn render_confirm_exit(frame: &mut Frame<'_>, theme: &Theme) {
    let area = centered_rect(30, 20, frame.area());
    let paragraph = Paragraph::new("Quit Rudo? (y/n)")
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.foreground))
        .block(
            Block::bordered()
                .title(" Confirm Exit ")
                .border_type(BorderType::Rounded)
                .fg(theme.red),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

pub fn render_esc_menu(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let theme = &app_state.theme;
    let area = centered_rect(40, 40, frame.area());
    let items = ["Resume", "Keybinds", "Themes", "Exit"];
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();

    let list = List::new(list_items)
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(theme.selection_bg))
        .block(
            Block::bordered()
                .title(" Rudo Menu ")
                .border_type(BorderType::Rounded)
                .fg(theme.accent),
        );

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut app_state.esc_menu_state);
}

pub fn render_theme_picker(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let theme = &app_state.theme;
    let area = centered_rect(40, 50, frame.area());
    let items = [
        Theme::catppuccin_mocha().name,
        Theme::catppuccin_macchiato().name,
        Theme::catppuccin_frappe().name,
        Theme::catppuccin_latte().name,
        Theme::gruvbox().name,
        Theme::rose_pine().name,
        Theme::solarized_dark().name,
    ];
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(i.clone())).collect();

    let list = List::new(list_items)
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(theme.selection_bg))
        .block(
            Block::bordered()
                .title(" Themes ")
                .border_type(BorderType::Rounded)
                .fg(theme.accent),
        );

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut app_state.theme_picker_state);
}

pub fn render_keybinds(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let theme = &app_state.theme;
    let area = centered_rect(55, 70, frame.area());

    let keybinds: &[(&str, &str)] = &[
        ("── Items Panel ────────────", ""),
        ("i", "Add item"),
        ("s", "Add sub-item"),
        ("h", "Add header"),
        ("-", "Add separator"),
        ("E", "Edit selected"),
        ("d", "Delete selected"),
        ("u", "Uncheck item"),
        ("Enter", "Toggle check"),
        ("Backspace", "Step uncheck"),
        ("J/K", "Reorder items"),
        ("── Navigation ────────────", ""),
        ("j/k", "Move cursor"),
        ("g/G", "Top / Bottom"),
        ("1", "Focus items"),
        ("2", "Focus lists"),
        ("Tab", "Switch panel"),
        ("── App ───────────────────", ""),
        ("/", "Search"),
        ("v", "Toggle completed"),
        ("f", "Toggle folder (lists)"),
        ("q", "Quit"),
        ("Esc", "Open menu"),
    ];

    let lines: Vec<Line> = keybinds
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() {
                Line::from(Span::styled(*key, Style::default().fg(theme.gray)))
            } else {
                Line::from(vec![
                    Span::styled(format!("{:12}", key), Style::default().fg(theme.accent).bold()),
                    Span::styled(*desc, Style::default().fg(theme.foreground)),
                ])
            }
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(
        Block::bordered()
            .title(" Keybinds ")
            .border_type(BorderType::Rounded)
            .fg(theme.accent),
    );

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
