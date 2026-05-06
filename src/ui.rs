use crate::app::Appstate;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Stylize;
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::ToSpan;
use ratatui::widgets::{Block, BorderType, List, ListItem, Padding, Paragraph, block};

pub fn render(frame: &mut Frame, app_state: &mut Appstate) {
    if app_state.is_add_new {
        render_input_from(frame, app_state);
    } else {
        render_list(frame, app_state);
    }
}

pub fn render_input_from(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let input_paragraph = Paragraph::new(app_state.input_value.as_str());
    let border_area = frame.area();
    let chuncks = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area);

    if chuncks.is_empty() {
        let error_message = Paragraph::new("No area available to render input block");
        frame.render_widget(error_message, frame.area());
        return;
    }

    let inner_area = chuncks[0];

    let input_block = Block::bordered()
        .title(" Input description ".to_span().into_centered_line())
        .fg(Color::Rgb(116, 199, 236))
        .border_type(BorderType::Rounded);
    frame.render_widget(input_block, border_area);
    frame.render_widget(input_paragraph, inner_area);
}

pub fn render_list(frame: &mut Frame<'_>, app_state: &mut Appstate) {
    let border_area = frame.area();

    let chuncks = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area);

    if chuncks.is_empty() {
        return;
    }

    let inner_area = chuncks[0];

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
