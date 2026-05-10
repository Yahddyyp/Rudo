use crate::app::state::{AppMode, Appstate, EditTarget, ItemType, ListGroupType, ListType, Panel, TodoItem, TodoList};
use crate::app::state::FormAction;
use crate::app::handlers::{handle_add_new, handle_key};
use crate::events::{handle_delete_confirmation, handle_esc_menu, handle_theme_picker};
use crate::ui::render;
use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind,
};
use ratatui::crossterm::execute;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Rect;

mod app;
mod cli;
mod events;
mod io;
mod theme;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    use clap::Parser;
    let cli = cli::Cli::parse();
    match cli.command {
        Some(cmd) => { cli::run_cli(cmd); Ok(()) }
        None => launch_tui(),
    }
}

fn launch_tui() -> Result<()> {
    let mut terminal = ratatui::init();
    execute!(terminal.backend_mut(), EnableMouseCapture)?;

    let mut state = match io::load_state() {
        Some(mut loaded) => {
            loaded.theme = loaded.get_theme();
            if !loaded.lists.is_empty() {
                loaded.list_state.select(Some(0));
                loaded.lists_list_state.select(Some(0));
            }
            loaded
        }
        None => {
            let mut state = Appstate::default();
            state.lists.push(TodoList {
                id: 0,
                name: "Untitled".to_string(),
                items: Vec::new(),
                list_type: ListType::List,
                group: ListGroupType::None,
                parent_id: None,
            });
            state.current_list_index = 0;
            state.list_state.select(Some(0));
            state.lists_list_state.select(Some(0));
            state
        }
    };

    let result = run(&mut terminal, &mut state);

    let _ = io::save_state(&state);
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal, app_state: &mut Appstate) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        match event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Esc
                    && app_state.mode == AppMode::Normal
                    && !app_state.is_add_new
                    && !app_state.confirming_delete
                {
                    app_state.mode = AppMode::EscMenu;
                    app_state.esc_menu_state.select(Some(0));
                    continue;
                }

                match app_state.mode {
                    AppMode::Normal => handle_normal_mode(key, app_state)?,
                    AppMode::EscMenu => {
                        if handle_esc_menu(key, app_state) {
                            break;
                        }
                    }
                    AppMode::ThemePicker => handle_theme_picker(key, app_state),
                    AppMode::Keybinds => {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                            app_state.mode = AppMode::EscMenu;
                            app_state.esc_menu_state.select(Some(1));
                        }
                    }
                    AppMode::ConfirmExit => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(()),
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app_state.mode = AppMode::Normal;
                        }
                        _ => {}
                    },
                    AppMode::Search => handle_search_mode(key, app_state),
                }
            }
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Down(event::MouseButton::Left) {
                    let size = terminal.size()?;
                    let main_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                        .split(Rect::new(0, 0, size.width, size.height));

                    let click = Rect::new(mouse.column, mouse.row, 1, 1);
                    if main_chunks[0].intersects(click) {
                        app_state.active_panel = Panel::List;
                    } else if main_chunks[1].intersects(click) {
                        app_state.active_panel = Panel::NewList;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_normal_mode(
    key: ratatui::crossterm::event::KeyEvent,
    app_state: &mut Appstate,
) -> Result<()> {
    if app_state.confirming_delete {
        handle_delete_confirmation(key, app_state);
    } else if app_state.is_add_new {
        match handle_add_new(key, app_state) {
            FormAction::None => {}
            FormAction::Submit => {
                app_state.is_add_new = false;
                if app_state.is_editing {
                    apply_edit(app_state);
                } else if app_state.active_panel == Panel::List {
                    if let Some(list) =
                        app_state.lists.get_mut(app_state.current_list_index)
                    {
                        list.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                            sub_items: Vec::new(),
                            item_type: ItemType::Task,
                        });
                    }
                    app_state.input_value.clear();
                } else if app_state.active_panel == Panel::NewList {
                    let new_id = app_state.lists.len() + 1;
                    app_state.lists.push(TodoList {
                        id: new_id,
                        name: app_state.input_value.clone(),
                        items: Vec::new(),
                        list_type: ListType::List,
                        group: ListGroupType::None,
                        parent_id: app_state.current_folder_id,
                    });
                    app_state.input_value.clear();
                }
            }
            FormAction::Escape => {
                app_state.is_add_new = false;
                app_state.is_editing = false;
            }
        }
    } else if handle_key(key, app_state) {
        std::process::exit(0);
    }
    Ok(())
}

fn apply_edit(app_state: &mut Appstate) {
    if app_state.edit_target == EditTarget::Item {
        if let Some(visual_idx) = app_state.list_state.selected() {
            if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                let mut pos = 0;
                let mut main_idx = 0;
                let mut sub_idx = 0;
                let mut is_sub = false;
                'outer: for (mi, item) in list.items.iter().enumerate() {
                    if pos == visual_idx {
                        main_idx = mi;
                        break;
                    }
                    pos += 1;
                    for (si, _) in item.sub_items.iter().enumerate() {
                        if pos == visual_idx {
                            main_idx = mi;
                            sub_idx = si;
                            is_sub = true;
                            break 'outer;
                        }
                        pos += 1;
                    }
                }
                if let Some(item) = list.items.get_mut(main_idx) {
                    if is_sub {
                        if let Some(sub) = item.sub_items.get_mut(sub_idx) {
                            sub.description = app_state.input_value.clone();
                        }
                    } else {
                        item.description = app_state.input_value.clone();
                    }
                }
            }
        }
    } else if app_state.edit_target == EditTarget::ListName {
        if let Some(index) = app_state.lists_list_state.selected() {
            if let Some(list) = app_state.lists.get_mut(index) {
                list.name = app_state.input_value.clone();
            }
        }
    }
    app_state.is_editing = false;
    app_state.edit_target = EditTarget::None;
    app_state.editing_sub_index = None;
}

fn handle_search_mode(key: ratatui::crossterm::event::KeyEvent, app_state: &mut Appstate) {
    match key.code {
        KeyCode::Esc => {
            app_state.mode = AppMode::Normal;
            app_state.search_input_value.clear();
        }
        KeyCode::Enter => {
            app_state.mode = AppMode::Normal;
        }
        KeyCode::Char(c) => {
            app_state
                .search_input_value
                .insert(app_state.search_cursor_position, c);
            app_state.search_cursor_position += 1;
        }
        KeyCode::Backspace => {
            if app_state.search_cursor_position > 0 {
                app_state
                    .search_input_value
                    .remove(app_state.search_cursor_position - 1);
                app_state.search_cursor_position -= 1;
            }
        }
        KeyCode::Left => {
            if app_state.search_cursor_position > 0 {
                app_state.search_cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            if app_state.search_cursor_position < app_state.search_input_value.len() {
                app_state.search_cursor_position += 1;
            }
        }
        _ => {}
    }
}
