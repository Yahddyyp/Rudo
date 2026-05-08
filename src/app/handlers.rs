use super::super::actions::FormAction;
use super::state::{Appstate, EditTarget, Panel, TodoItem};
use ratatui::{
    crossterm::event::{self, KeyEvent},
    widgets::ListState,
};

pub fn handle_add_new(key: KeyEvent, app_state: &mut Appstate) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.insert(app_state.cursor_position, c);
            app_state.cursor_position += 1;
        }
        event::KeyCode::Backspace => {
            if app_state.cursor_position > 0 {
                app_state.input_value.remove(app_state.cursor_position - 1);
                app_state.cursor_position -= 1;
            }
        }
        event::KeyCode::Left => {
            if app_state.cursor_position > 0 {
                app_state.cursor_position -= 1;
            }
        }
        event::KeyCode::Right => {
            if app_state.cursor_position < app_state.input_value.len() {
                app_state.cursor_position += 1;
            }
        }
        event::KeyCode::Home => {
            app_state.cursor_position = 0;
        }
        event::KeyCode::End => {
            app_state.cursor_position = app_state.input_value.len();
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

pub fn handle_key(key: KeyEvent, app_state: &mut Appstate) -> bool {
    match key.code {
        event::KeyCode::Backspace => {
            if app_state.active_panel == Panel::List {
                if let Some(visual_idx) = app_state.list_state.selected() {
                    if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                        let mut pos = 0;
                        let mut main_idx = 0;
                        for (mi, item) in list.items.iter().enumerate() {
                            if pos == visual_idx {
                                main_idx = mi;
                                break;
                            }
                            pos += 1;
                            for _ in &item.sub_items {
                                if pos == visual_idx {
                                    main_idx = mi;
                                    break;
                                }
                                pos += 1;
                            }
                        }
                        if let Some(item) = list.items.get_mut(main_idx) {
                            if !item.sub_items.is_empty() {
                                let completed = item.sub_items.iter().filter(|s| s.is_done).count();
                                if completed > 0 {
                                    if let Some(sub) = item.sub_items.get_mut(completed - 1) {
                                        sub.is_done = false;
                                    }
                                    if completed == item.sub_items.len() {
                                        item.is_done = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        event::KeyCode::Enter => {
            if app_state.active_panel == Panel::NewList {
                if let Some(index) = app_state.lists_list_state.selected() {
                    app_state.current_list_index = index;
                    app_state.list_state.select(Some(0));
                    app_state.active_panel = Panel::List;
                }
            } else {
                if let Some(visual_idx) = app_state.list_state.selected() {
                    if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
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
                                    sub.is_done = !sub.is_done;
                                }
                            } else if item.sub_items.is_empty() {
                                item.is_done = !item.is_done;
                            } else {
                                let completed = item.sub_items.iter().filter(|s| s.is_done).count();
                                let total = item.sub_items.len();
                                if completed < total {
                                    if let Some(sub) = item.sub_items.get_mut(completed) {
                                        sub.is_done = true;
                                    }
                                } else if completed > 0 {
                                    if let Some(sub) = item.sub_items.get_mut(completed - 1) {
                                        sub.is_done = false;
                                    }
                                }
                                let new_completed = item.sub_items.iter().filter(|s| s.is_done).count();
                                item.is_done = new_completed == total;
                            }
                        }
                    }
                }
            }
        }

        event::KeyCode::Char(char) => match char {
            'q' => return true,
            'i' => {
                if app_state.active_panel == Panel::List {
                    app_state.is_add_new = true;
                    app_state.is_editing = false;
                    app_state.edit_target = EditTarget::Item;
                    app_state.input_value.clear();
                    app_state.cursor_position = 0;
                } else if app_state.active_panel == Panel::NewList {
                    app_state.is_add_new = true;
                    app_state.is_editing = false;
                    app_state.edit_target = EditTarget::ListName;
                    app_state.input_value.clear();
                    app_state.cursor_position = 0;
                }
            }
            's' => {
                if app_state.active_panel == Panel::List {
                    if let Some(visual_idx) = app_state.list_state.selected() {
                        if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                            let mut pos = 0;
                            let mut main_idx = 0;
                            for (mi, item) in list.items.iter().enumerate() {
                                if pos == visual_idx {
                                    main_idx = mi;
                                    break;
                                }
                                pos += 1;
                                for _ in &item.sub_items {
                                    if pos == visual_idx {
                                        main_idx = mi;
                                        break;
                                    }
                                    pos += 1;
                                }
                            }
                            if let Some(item) = list.items.get_mut(main_idx) {
                                item.sub_items.push(TodoItem {
                                    is_done: false,
                                    description: "new subtask".to_string(),
                                    sub_items: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
            'E' => {
                app_state.is_add_new = true;
                app_state.is_editing = true;
                app_state.edit_target = EditTarget::Item;
                app_state.input_value.clear();
                app_state.cursor_position = 0;

                if app_state.active_panel == Panel::List {
                    if let Some(visual_idx) = app_state.list_state.selected() {
                        if let Some(list) = app_state.lists.get(app_state.current_list_index) {
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
                            if let Some(item) = list.items.get(main_idx) {
                                if is_sub {
                                    if let Some(sub) = item.sub_items.get(sub_idx) {
                                        app_state.input_value = sub.description.clone();
                                        app_state.cursor_position = app_state.input_value.len();
                                    }
                                } else {
                                    app_state.input_value = item.description.clone();
                                    app_state.cursor_position = app_state.input_value.len();
                                }
                            }
                        }
                    }
                }
            }
            'd' => {
                if app_state.active_panel == Panel::List {
                    if let Some(visual_idx) = app_state.list_state.selected() {
                        if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                            let mut pos = 0;
                            let mut main_idx = 0;
                            for (mi, item) in list.items.iter().enumerate() {
                                if pos == visual_idx {
                                    main_idx = mi;
                                    break;
                                }
                                pos += 1;
                                for _ in &item.sub_items {
                                    if pos == visual_idx {
                                        main_idx = mi;
                                        break;
                                    }
                                    pos += 1;
                                }
                            }
                            list.items.remove(main_idx);
                        }
                    }
                } else if let Some(index) = app_state.lists_list_state.selected() {
                    app_state.lists.remove(index);
                    if app_state.current_list_index >= app_state.lists.len() && !app_state.lists.is_empty() {
                        app_state.current_list_index = app_state.lists.len() - 1;
                    }
                }
            }
            'g' => {
                app_state.g_count += 1;
                if app_state.g_count == 2 {
                    app_state.list_state.select(Some(0));
                    app_state.g_count = 0;
                }
            }
            'G' => {
                if app_state.active_panel == Panel::List {
                    if let Some(list) = app_state.lists.get(app_state.current_list_index) {
                        let mut count = 0;
                        for item in &list.items {
                            count += 1;
                            count += item.sub_items.len();
                        }
                        if count > 0 {
                            app_state.list_state.select(Some(count - 1));
                        }
                    }
                } else if !app_state.lists.is_empty() {
                    app_state.lists_list_state.select(Some(app_state.lists.len() - 1));
                }
            }
            'J' => {
                if app_state.active_panel == Panel::List {
                    if let Some(visual_idx) = app_state.list_state.selected() {
                        if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                            let mut pos = 0;
                            let mut main_idx = 0;
                            for (mi, item) in list.items.iter().enumerate() {
                                if pos == visual_idx {
                                    main_idx = mi;
                                    break;
                                }
                                pos += 1;
                                for _ in &item.sub_items {
                                    if pos == visual_idx {
                                        main_idx = mi;
                                        break;
                                    }
                                    pos += 1;
                                }
                            }
                            if main_idx < list.items.len() - 1 {
                                list.items.swap(main_idx, main_idx + 1);
                                let mut new_pos = visual_idx + 1;
                                for i in 0..main_idx {
                                    new_pos += 1;
                                    new_pos += list.items[i].sub_items.len();
                                }
                                app_state.list_state.select(Some(new_pos));
                            }
                        }
                    }
                } else if app_state.active_panel == Panel::NewList {
                    if let Some(index) = app_state.lists_list_state.selected() {
                        if index < app_state.lists.len() - 1 {
                            app_state.lists.swap(index, index + 1);
                            app_state.lists_list_state.select(Some(index + 1));
                            if app_state.current_list_index == index {
                                app_state.current_list_index = index + 1;
                            } else if app_state.current_list_index == index + 1 {
                                app_state.current_list_index = index;
                            }
                        }
                    }
                }
            }
            'K' => {
                if app_state.active_panel == Panel::List {
                    if let Some(visual_idx) = app_state.list_state.selected() {
                        if visual_idx > 0 {
                            if let Some(list) = app_state.lists.get_mut(app_state.current_list_index) {
                                let mut pos = 0;
                                let mut main_idx = 0;
                                for (mi, item) in list.items.iter().enumerate() {
                                    if pos == visual_idx {
                                        main_idx = mi;
                                        break;
                                    }
                                    pos += 1;
                                    for _ in &item.sub_items {
                                        if pos == visual_idx {
                                            main_idx = mi;
                                            break;
                                        }
                                        pos += 1;
                                    }
                                }
                                if main_idx > 0 {
                                    list.items.swap(main_idx, main_idx - 1);
                                    let mut new_pos = visual_idx - 1;
                                    for i in 0..main_idx - 1 {
                                        new_pos += 1;
                                        new_pos += list.items[i].sub_items.len();
                                    }
                                    app_state.list_state.select(Some(new_pos));
                                }
                            }
                        }
                    }
                } else if app_state.active_panel == Panel::NewList {
                    if let Some(index) = app_state.lists_list_state.selected() {
                        if index > 0 {
                            app_state.lists.swap(index, index - 1);
                            app_state.lists_list_state.select(Some(index - 1));
                            if app_state.current_list_index == index {
                                app_state.current_list_index = index - 1;
                            } else if app_state.current_list_index == index - 1 {
                                app_state.current_list_index = index;
                            }
                        }
                    }
                }
            }
            'n' => {
                if app_state.active_panel == Panel::List {
                    app_state.is_add_new = true;
                    app_state.is_editing = false;
                    app_state.edit_target = EditTarget::ListName;
                    app_state.input_value.clear();
                    app_state.cursor_position = 0;
                }
            }
            '1' => {
                app_state.active_panel = Panel::List;
                app_state.g_count = 0;
            }
            '2' => {
                app_state.active_panel = Panel::NewList;
                app_state.g_count = 0;
            }
            'j' => {
                if app_state.active_panel == Panel::List {
                    if let Some(list) = app_state.lists.get(app_state.current_list_index) {
                        let current = app_state.list_state.selected().unwrap_or(0);
                        let mut pos = 0;
                        for (mi, item) in list.items.iter().enumerate() {
                            if pos == current {
                                if !item.sub_items.is_empty() {
                                    app_state.list_state.select(Some(current + 1));
                                } else if mi < list.items.len() - 1 {
                                    app_state.list_state.select_next();
                                }
                                return false;
                            }
                            pos += 1;
                            for _ in &item.sub_items {
                                if pos == current {
                                    if pos < current + item.sub_items.len() {
                                        app_state.list_state.select_next();
                                    } else if mi < list.items.len() - 1 {
                                        pos += 1;
                                        for ni in (mi + 1)..list.items.len() {
                                            pos += 1;
                                            pos += list.items[ni].sub_items.len();
                                        }
                                        app_state.list_state.select(Some(pos));
                                    }
                                    return false;
                                }
                                pos += 1;
                            }
                        }
                        if pos == current && current > 0 {
                            app_state.list_state.select_next();
                        }
                    }
                } else {
                    app_state.lists_list_state.select_next();
                }
            }
            'k' => {
                if app_state.active_panel == Panel::List {
                    if let Some(list) = app_state.lists.get(app_state.current_list_index) {
                        let current = app_state.list_state.selected().unwrap_or(0);
                        if current == 0 {
                            return false;
                        }
                        let mut pos = 0;
                        for (_mi, item) in list.items.iter().enumerate() {
                            if pos == current {
                                app_state.list_state.select_previous();
                                return false;
                            }
                            pos += 1;
                            for (si, _) in item.sub_items.iter().enumerate() {
                                if pos == current {
                                    app_state.list_state.select_previous();
                                    return false;
                                }
                                pos += 1;
                            }
                        }
                        app_state.list_state.select_previous();
                    }
                } else {
                    app_state.lists_list_state.select_previous();
                }
            }
            _ => {
                app_state.g_count = 0;
            }
        },

        _ => {}
    }
    false
}