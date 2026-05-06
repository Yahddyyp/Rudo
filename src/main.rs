mod actions;
mod app;
mod ui;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::app::{Appstate, TodoItem, handle_add_new, handle_key};
use crate::ui::render;

fn main() -> Result<()> {
    let mut state = Appstate::default();
    state.is_add_new = false;
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut Appstate) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    actions::FormAction::None => {}
                    actions::FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                        app_state.list_state.select(Some(app_state.items.len() - 1));
                    }
                    actions::FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}
