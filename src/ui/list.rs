use crate::app::state::{Appstate, ItemType, ListGroupType, ListType, Panel, TodoItem};
use ratatui::layout::Rect;
use ratatui::prelude::{Alignment, Stylize};
use ratatui::style::Style;
use ratatui::text::{Line, Span, ToSpan};
use ratatui::widgets::{Block, BorderType, List, ListItem, Padding, Paragraph};
use ratatui::Frame;

pub fn render_list(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let theme = &app_state.theme;
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

    let (done_count, task_count) = match current_list {
        Some(list) => {
            let tasks: Vec<_> = list
                .items
                .iter()
                .filter(|i| i.item_type == ItemType::Task)
                .collect();
            let done = tasks.iter().filter(|i| i.is_done).count();
            (done, tasks.len())
        }
        None => (0, 0),
    };
    let title_str = if task_count > 0 {
        format!(" {} [{}/{}] ", list_name, done_count, task_count)
    } else {
        format!(" {} ", list_name)
    };

    let search = app_state.search_input_value.to_lowercase();
    let filtered_items: Vec<&TodoItem> = match current_list {
        Some(list) => list
            .items
            .iter()
            .filter(|item| {
                let visible = if app_state.show_completed {
                    true
                } else {
                    !item.is_done
                        || item.item_type == ItemType::Header
                        || item.item_type == ItemType::Separator
                };
                if !visible {
                    return false;
                }
                if search.is_empty() {
                    return true;
                }
                item.description.to_lowercase().contains(&search)
                    || item
                        .sub_items
                        .iter()
                        .any(|s| s.description.to_lowercase().contains(&search))
            })
            .collect(),
        None => Vec::new(),
    };

    let items: Vec<ListItem> = if !filtered_items.is_empty() {
        filtered_items
            .into_iter()
            .flat_map(|item| {
                if item.item_type == ItemType::Separator {
                    vec![ListItem::new(
                        Line::from("─────────────────────────────────────────────")
                            .style(Style::default().fg(theme.gray)),
                    )]
                } else if item.item_type == ItemType::Header {
                    vec![ListItem::new(
                        Line::from(vec![
                            Span::raw(" "),
                            Span::styled(
                                item.description.clone(),
                                Style::default().fg(theme.accent2).bold(),
                            ),
                        ])
                        .alignment(Alignment::Center),
                    )]
                } else {
                    let count_str = if item.sub_items.is_empty() {
                        String::new()
                    } else {
                        let completed = item.sub_items.iter().filter(|s| s.is_done).count();
                        let total = item.sub_items.len();
                        format!(" ({}/{})", completed, total)
                    };
                    let item_style = if item.is_done {
                        Style::default().fg(theme.gray)
                    } else {
                        Style::default().fg(theme.foreground)
                    };
                    let checkbox = if item.is_done {
                        Span::styled("[✓]", Style::default().fg(theme.green))
                    } else {
                        Span::styled("[ ]", Style::default().fg(theme.gray))
                    };
                    let text_span = Span::styled(
                        format!(" {}{}", item.description, count_str),
                        item_style,
                    );
                    let main_item = ListItem::new(Line::from(vec![checkbox, text_span]));

                    let sub_items: Vec<ListItem> = item
                        .sub_items
                        .iter()
                        .map(|sub| {
                            let sub_checkbox = if sub.is_done {
                                Span::styled("  [✓]", Style::default().fg(theme.green))
                            } else {
                                Span::styled("  [ ]", Style::default().fg(theme.gray))
                            };
                            let sub_style = if sub.is_done {
                                Style::default().fg(theme.gray)
                            } else {
                                Style::default().fg(theme.foreground)
                            };
                            let sub_text =
                                Span::styled(format!(" {}", sub.description), sub_style);
                            ListItem::new(Line::from(vec![sub_checkbox, sub_text]))
                        })
                        .collect();

                    std::iter::once(main_item).chain(sub_items).collect()
                }
            })
            .collect()
    } else {
        let help_text = if search.is_empty() {
            "No items  ·  press i to add one"
        } else {
            "No matches found"
        };
        frame.render_widget(
            Paragraph::new(help_text)
                .alignment(ratatui::layout::Alignment::Center)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .padding(Padding::horizontal(1))
                        .title(title_str.to_span().into_centered_line())
                        .fg(theme.gray),
                ),
            area,
        );
        return;
    };

    let is_active = app_state.active_panel == Panel::List;
    let list = List::new(items)
        .highlight_symbol("❯ ")
        .highlight_style(
            Style::default()
                .bg(theme.selection_bg)
                .fg(theme.foreground)
                .bold(),
        )
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .padding(Padding::horizontal(1))
                .title(title_str.to_span().into_centered_line())
                .fg(if is_active { theme.accent } else { theme.gray }),
        );
    frame.render_stateful_widget(list, area, &mut app_state.list_state);
}

pub fn render_list_list(frame: &mut Frame<'_>, area: Rect, app_state: &mut Appstate) {
    let theme = &app_state.theme;
    let current_folder_id = app_state.current_folder_id;

    let mut items: Vec<ListItem> = Vec::new();

    if current_folder_id.is_some() {
        items.push(ListItem::new("..").style(Style::default().fg(theme.gray)));
    }

    let filtered_lists: Vec<(usize, &crate::app::state::TodoList)> = app_state
        .lists
        .iter()
        .enumerate()
        .filter(|(_, list)| list.parent_id == current_folder_id)
        .collect();

    for (i, list) in filtered_lists {
        let display_index = if current_folder_id.is_some() { i + 1 } else { i };
        let is_selected =
            display_index == app_state.lists_list_state.selected().unwrap_or(0);
        let suffix = if is_selected { " •" } else { "" };

        match list.group {
            ListGroupType::Separator => {
                items.push(
                    ListItem::new("─────────────────")
                        .style(Style::default().fg(theme.gray).bold()),
                );
            }
            ListGroupType::None => {
                if list.list_type == ListType::Folder {
                    items.push(
                        ListItem::new(format!("[Folder] {}{}", list.name, suffix))
                            .style(Style::default().fg(theme.accent)),
                    );
                } else {
                    items.push(ListItem::new(format!("{}{}", list.name, suffix)));
                }
            }
        }
    }

    let list = List::new(items)
        .highlight_symbol("> ")
        .highlight_style(Style::default().bg(theme.selection_bg))
        .block(
            Block::bordered()
                .title(" Lists ")
                .fg(if app_state.active_panel == Panel::NewList {
                    theme.accent
                } else {
                    theme.gray
                }),
        );
    frame.render_stateful_widget(list, area, &mut app_state.lists_list_state);
}
