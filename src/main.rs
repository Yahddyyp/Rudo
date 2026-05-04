use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    prelude::{Color, Stylize},
    style::Style,
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct Appstate {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

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
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                        app_state.list_state.select(Some(app_state.items.len() - 1));
                    }
                    FormAction::Escape => {
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

fn handle_add_new(key: KeyEvent, app_state: &mut Appstate) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut Appstate) -> bool {
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

fn render(frame: &mut Frame, app_state: &mut Appstate) {
    if app_state.is_add_new {
        render_input_from(frame, app_state);
    } else {
        render_list(frame, app_state);
    }
}

fn render_input_from(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    Paragraph::new(app_state.input_value.as_str())
        .block(
            Block::bordered()
                .title(" Input description ".to_span().into_centered_line())
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        )
        .render(frame.area(), frame.buffer_mut());
}

fn render_list(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let border_area = frame.area();

    let inner_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area)[0];

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Rudo ".to_span().into_centered_line())
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(app_state.items.iter().map(|x| {
        let value = if x.is_done {
            x.description.to_span().crossed_out()
        } else {
            x.description.to_span()
        };
        ListItem::from(value)
    }))
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
