use crate::app::state::Appstate;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

pub fn render_status(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let theme = &app_state.theme;
    let mut total_items = 0;
    let mut completed_items = 0;
    let mut total_sub_items = 0;
    let mut completed_sub_items = 0;

    for list in &app_state.lists {
        for item in &list.items {
            total_items += 1;
            if item.is_done {
                completed_items += 1;
            }
            for sub in &item.sub_items {
                total_sub_items += 1;
                if sub.is_done {
                    completed_sub_items += 1;
                }
            }
        }
    }

    let pending_items = total_items - completed_items;
    let item_percentage = if total_items > 0 {
        (completed_items as f32 / total_items as f32) * 100.0
    } else {
        0.0
    };

    let pending_sub_items = total_sub_items - completed_sub_items;
    let sub_item_percentage = if total_sub_items > 0 {
        (completed_sub_items as f32 / total_sub_items as f32) * 100.0
    } else {
        0.0
    };

    let total_tasks = total_items + total_sub_items;
    let completed_tasks = completed_items + completed_sub_items;
    let overall_percentage = if total_tasks > 0 {
        (completed_tasks as f32 / total_tasks as f32) * 100.0
    } else {
        0.0
    };

    let cat_art: Vec<Line> = vec![
        Line::from(Span::styled(r"  /\_/\  ", Style::default().fg(theme.accent))),
        Line::from(vec![
            Span::styled(" ( ", Style::default().fg(theme.accent)),
            Span::styled("^.^", Style::default().fg(theme.accent2).bold()),
            Span::styled(" ) ", Style::default().fg(theme.accent)),
        ]),
        Line::from(vec![
            Span::styled("  > ", Style::default().fg(theme.accent)),
            Span::styled("♥", Style::default().fg(theme.red)),
            Span::styled(" <  ", Style::default().fg(theme.accent)),
        ]),
        Line::from(Span::styled(" /     \\ ", Style::default().fg(theme.accent))),
        Line::from(Span::styled("(_)   (_)", Style::default().fg(theme.accent))),
        Line::from(Span::raw("         ")),
        Line::from(Span::styled(" meow~  ", Style::default().fg(theme.accent2))),
    ];

    let motivation = match overall_percentage as u32 {
        0 => "Let's get started!",
        1..=25 => "You can do it!",
        26..=50 => "Keep going!",
        51..=75 => "More than halfway!",
        76..=99 => "Almost there!",
        _ => "All done! Great job!",
    };

    let overall_color = if overall_percentage >= 80.0 {
        theme.green
    } else if overall_percentage >= 40.0 {
        theme.accent
    } else {
        theme.accent2
    };

    let bar_width: usize = 14;

    let make_bar = |pct: f32, width: usize, fill: Color, empty: Color| -> Vec<Span<'static>> {
        let filled = ((pct / 100.0) * width as f32).round() as usize;
        let empty_count = width.saturating_sub(filled);
        vec![
            Span::styled("▰".repeat(filled), Style::default().fg(fill)),
            Span::styled("▱".repeat(empty_count), Style::default().fg(empty)),
        ]
    };

    let divider = Line::from(Span::styled(
        "─────────────────────",
        Style::default().fg(theme.gray),
    ));

    let mut stats_lines: Vec<Line> = vec![];

    stats_lines.push(Line::from(vec![
        Span::styled("LISTS  ", Style::default().fg(theme.accent).bold()),
        Span::styled(
            format!("{} total", app_state.lists.len()),
            Style::default().fg(theme.foreground),
        ),
    ]));
    stats_lines.push(divider.clone());

    let item_color = if item_percentage >= 80.0 {
        theme.green
    } else {
        theme.accent2
    };
    stats_lines.push(Line::from(vec![
        Span::styled("ITEMS  ", Style::default().fg(theme.accent).bold()),
        Span::styled(
            format!("{:.0}%", item_percentage),
            Style::default().fg(item_color),
        ),
    ]));
    stats_lines.push(Line::from(make_bar(item_percentage, bar_width, item_color, theme.gray)));
    stats_lines.push(Line::from(vec![
        Span::styled(format!("  ✓ {} done  ", completed_items), Style::default().fg(theme.green)),
        Span::styled(format!("○ {} left", pending_items), Style::default().fg(theme.gray)),
    ]));
    stats_lines.push(Line::from(""));

    let sub_color = if sub_item_percentage >= 80.0 {
        theme.green
    } else {
        theme.accent2
    };
    stats_lines.push(Line::from(vec![
        Span::styled("SUB    ", Style::default().fg(theme.accent).bold()),
        Span::styled(
            format!("{:.0}%", sub_item_percentage),
            Style::default().fg(sub_color),
        ),
    ]));
    stats_lines.push(Line::from(make_bar(sub_item_percentage, bar_width, sub_color, theme.gray)));
    stats_lines.push(Line::from(vec![
        Span::styled(format!("  ✓ {} done  ", completed_sub_items), Style::default().fg(theme.green)),
        Span::styled(format!("○ {} left", pending_sub_items), Style::default().fg(theme.gray)),
    ]));
    stats_lines.push(Line::from(""));
    stats_lines.push(divider.clone());

    stats_lines.push(Line::from(vec![
        Span::styled("TOTAL  ", Style::default().fg(theme.accent).bold()),
        Span::styled(format!("{} tasks", total_tasks), Style::default().fg(theme.foreground)),
    ]));
    stats_lines.push(Line::from(make_bar(overall_percentage, bar_width, overall_color, theme.gray)));
    stats_lines.push(Line::from(vec![
        Span::styled(format!("  {}/{} ", completed_tasks, total_tasks), Style::default().fg(overall_color).bold()),
        Span::styled(format!("({:.0}% done)", overall_percentage), Style::default().fg(theme.gray)),
    ]));
    stats_lines.push(Line::from(""));
    stats_lines.push(Line::from(Span::styled(motivation, Style::default().fg(overall_color).bold())));

    let stats_paragraph = Paragraph::new(stats_lines);

    let status_block = Block::bordered()
        .title(" Status ")
        .border_type(BorderType::Rounded)
        .fg(theme.accent2);

    let inner_area = status_block.inner(area);
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(inner_area);

    let cat_height = cat_art.len() as u16;
    let cat_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(
                status_chunks[1]
                    .height
                    .saturating_sub(cat_height)
                    .checked_div(2)
                    .unwrap_or(0),
            ),
            Constraint::Length(cat_height),
            Constraint::Min(0),
        ])
        .split(status_chunks[1])[1];

    let cat_paragraph = Paragraph::new(cat_art).style(Style::default().fg(theme.accent));

    frame.render_widget(status_block, area);
    frame.render_widget(stats_paragraph, status_chunks[0]);
    frame.render_widget(cat_paragraph, cat_area);
}
