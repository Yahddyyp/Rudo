use crate::actions::FormAction;
use ratatui::{
    crossterm::event::{self, KeyEvent},
    widgets::ListState,
};

#[derive(Debug, Default)]
pub struct Appstate {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub input_value: String,
}

#[derive(Debug, Default)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
}

pub fn handle_add_new(key: KeyEvent, app_state: &mut Appstate) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Enter => {
            return crate::actions::FormAction::Submit;
        }
        event::KeyCode::Esc => {
            return crate::actions::FormAction::Escape;
        }
        _ => {}
    }
    crate::actions::FormAction::None
}

pub fn handle_key(key: KeyEvent, app_state: &mut Appstate) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if app_state.items.get(index).is_some() {
                    if let Some(item) = app_state.items.get_mut(index) {
                        item.is_done = !item.is_done;
                    }
                }
            }
        }

        event::KeyCode::Char(char) => match char {
            'q' => {
                return true;
            }
            'i' => {
                app_state.is_add_new = true;
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'd' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'j' => {
                app_state.list_state.select_next();
            }
            _ => {}
        },
        _ => {}
    }
    false
}
