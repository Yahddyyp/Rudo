use crate::app::state::{AppMode, Appstate};
use crate::theme::Theme;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub fn handle_delete_confirmation(key: KeyEvent, app_state: &mut Appstate) {
    use crate::app::state::Panel;
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if app_state.active_panel == Panel::List {
                if let Some(visual_idx) = app_state.list_state.selected() {
                    if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                        let mut pos = 0;
                        let mut main_item_idx_to_delete = None;
                        let mut sub_item_idx_to_delete = None;

                        for (main_idx, item) in list.items.iter().enumerate() {
                            if pos == visual_idx {
                                main_item_idx_to_delete = Some(main_idx);
                                break;
                            }
                            let main_item_visual_start = pos;
                            pos += 1;

                            if visual_idx < pos + item.sub_items.len() {
                                sub_item_idx_to_delete = Some((
                                    main_idx,
                                    visual_idx - main_item_visual_start - 1,
                                ));
                                break;
                            }
                            pos += item.sub_items.len();
                        }

                        if let Some((main_idx, sub_idx)) = sub_item_idx_to_delete {
                            if let Some(item) = list.items.get_mut(main_idx) {
                                if sub_idx < item.sub_items.len() {
                                    item.sub_items.remove(sub_idx);
                                }
                            }
                        } else if let Some(main_idx) = main_item_idx_to_delete {
                            if main_idx < list.items.len() {
                                list.items.remove(main_idx);
                            }
                        }
                    }
                }
            } else if app_state.active_panel == Panel::NewList {
                if let Some(index) = app_state.lists_list_state.selected() {
                    if index < app_state.lists.len() {
                        app_state.lists.remove(index);
                        if app_state.current_list_index >= app_state.lists.len()
                            && !app_state.lists.is_empty()
                        {
                            app_state.current_list_index = app_state.lists.len() - 1;
                        }
                    }
                }
            }
            app_state.confirming_delete = false;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app_state.confirming_delete = false;
        }
        _ => {}
    }
}

pub fn handle_esc_menu(key: KeyEvent, app_state: &mut Appstate) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app_state.mode = AppMode::Normal;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let current = app_state.esc_menu_state.selected().unwrap_or(0);
            let next = if current >= 3 { 0 } else { current + 1 };
            app_state.esc_menu_state.select(Some(next));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let current = app_state.esc_menu_state.selected().unwrap_or(0);
            let next = if current == 0 { 3 } else { current - 1 };
            app_state.esc_menu_state.select(Some(next));
        }
        KeyCode::Enter => {
            if let Some(selected) = app_state.esc_menu_state.selected() {
                match selected {
                    0 => app_state.mode = AppMode::Normal,
                    1 => app_state.mode = AppMode::Keybinds,
                    2 => {
                        app_state.original_theme = Some(app_state.theme.clone());
                        app_state.mode = AppMode::ThemePicker;
                        app_state.theme_picker_state.select(Some(0));
                    }
                    3 => return true,
                    _ => {}
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_theme_picker(key: KeyEvent, app_state: &mut Appstate) {
    const THEMES: &[&str] = &[
        "Catppuccin Mocha",
        "Catppuccin Macchiato",
        "Catppuccin Frappé",
        "Catppuccin Latte",
        "Gruvbox",
        "Rosé Pine",
        "Solarized Dark",
    ];
    let themes_count = THEMES.len();

    match key.code {
        KeyCode::Esc => {
            if let Some(original_theme) = app_state.original_theme.take() {
                app_state.theme_name = original_theme.name.clone();
                app_state.theme = original_theme;
            }
            app_state.mode = AppMode::EscMenu;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let current = app_state.theme_picker_state.selected().unwrap_or(0);
            let next = if current >= themes_count - 1 { 0 } else { current + 1 };
            app_state.theme_name = THEMES[next].to_string();
            app_state.theme = Theme::from_name(&app_state.theme_name);
            app_state.theme_picker_state.select(Some(next));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let current = app_state.theme_picker_state.selected().unwrap_or(0);
            let next = if current == 0 { themes_count - 1 } else { current - 1 };
            app_state.theme_name = THEMES[next].to_string();
            app_state.theme = Theme::from_name(&app_state.theme_name);
            app_state.theme_picker_state.select(Some(next));
        }
        KeyCode::Enter => {
            app_state.original_theme = None;
            app_state.mode = AppMode::Normal;
        }
        _ => {}
    }
}
