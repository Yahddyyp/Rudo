mod list;
mod popups;
mod status;

pub use list::{render_list, render_list_list};
pub use popups::{render_input_from, render_search_input};
pub use status::render_status;

use crate::app::state::{AppMode, Appstate};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render(frame: &mut Frame, app_state: &mut Appstate) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[1]);

    render_list(frame, main_chunks[0], app_state);
    render_list_list(frame, right_chunks[0], app_state);
    render_status(frame, right_chunks[1], app_state);

    if app_state.is_add_new {
        render_input_from(frame, app_state);
    }

    if app_state.confirming_delete {
        popups::render_delete_confirmation(frame, &app_state.theme);
    }

    match app_state.mode {
        AppMode::EscMenu => popups::render_esc_menu(frame, app_state),
        AppMode::ThemePicker => popups::render_theme_picker(frame, app_state),
        AppMode::Keybinds => popups::render_keybinds(frame, app_state),
        AppMode::ConfirmExit => popups::render_confirm_exit(frame, &app_state.theme),
        AppMode::Search => render_search_input(frame, app_state),
        AppMode::Normal => {}
    }
}
