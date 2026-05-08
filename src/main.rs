mod actions;
mod app;
mod ui;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

use crate::app::{Appstate, EditTarget, Panel, TodoItem, TodoList, handle_add_new, handle_key};
use crate::ui::render;

fn main() -> Result<()> {
    let mut state = Appstate::default();
    state.cursor_position = 0;
    state.is_add_new = false;
    state.active_panel = Panel::List;
    state.lists.push(TodoList {
        name: "Untitled".to_string(),
        items: Vec::new(),
    });
    state.current_list_index = 0;
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

                        if app_state.is_editing {
                            if app_state.edit_target == EditTarget::Item {
                                if let Some(visual_idx) = app_state.list_state.selected() {
                                    if let Some(list) =
                                        app_state.lists.get_mut(app_state.current_list_index)
                                    {
                                        let mut pos = 0;
                                        let mut main_idx = 0;
                                        let mut sub_idx = 0;
                                        let mut is_sub = false;
                                        for (mi, item) in list.items.iter().enumerate() {
                                            if pos == visual_idx {
                                                main_idx = mi;
                                                is_sub = false;
                                                break;
                                            }
                                            pos += 1;
                                            for (si, _) in item.sub_items.iter().enumerate() {
                                                if pos == visual_idx {
                                                    main_idx = mi;
                                                    sub_idx = si;
                                                    is_sub = true;
                                                    break;
                                                }
                                                pos += 1;
                                            }
                                            if is_sub {
                                                break;
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
                        } else if app_state.active_panel == Panel::List {
                            if let Some(list) =
                                app_state.lists.get_mut(app_state.current_list_index)
                            {
                                list.items.push(TodoItem {
                                    is_done: false,
                                    description: app_state.input_value.clone(),
                                    sub_items: Vec::new(),
                                });
                            }
                        } else {
                            app_state.lists.push(TodoList {
                                name: app_state.input_value.clone(),
                                items: Vec::new(),
                            });
                            app_state.active_list_index = app_state.lists.len() - 1;
                        }

                        app_state.input_value.clear();
                        if let Some(list) = app_state.lists.get(app_state.current_list_index) {
                            if !list.items.is_empty() {
                                app_state.list_state.select(Some(list.items.len() - 1));
                            }
                        }
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
