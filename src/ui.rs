use crate::app::Appstate;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Stylize;
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, BorderType, List, ListItem, Padding, Paragraph};

pub fn render(frame: &mut Frame, app_state: &mut Appstate) {
    if app_state.is_add_new {
        render_input_from(frame, app_state);
    } else {
        render_list(frame, app_state);
    }
}

pub fn render_input_from(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    Paragraph::new(app_state.input_value.as_str())
        .block(
            Block::bordered()
                .title(" Input description ".to_span().into_centered_line())
                .fg(Color::Rgb(116, 199, 236))
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        )
        .render(frame.area(), frame.buffer_mut());
}

pub fn render_list(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let border_area = frame.area();

    let inner_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area)[0];

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Rudo ".to_span().into_centered_line())
        .fg(Color::Rgb((203), (166), (247)))
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
