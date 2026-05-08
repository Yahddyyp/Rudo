use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct Appstate {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub is_editing: bool,
    pub edit_target: EditTarget,
    pub editing_sub_index: Option<usize>,
    pub g_count: u8,
    pub input_value: String,
    pub cursor_position: usize,
    pub active_panel: Panel,
    pub lists: Vec<TodoList>,
    pub lists_list_state: ListState,
    pub active_list_index: usize,
    pub current_list_index: usize,
}

#[derive(Debug, Default)]
pub struct TodoList {
    pub name: String,
    pub items: Vec<TodoItem>,
}

#[derive(Debug, PartialEq)]
pub enum Panel {
    List,
    NewList,
}

#[derive(Debug, PartialEq)]
pub enum EditTarget {
    None,
    ListName,
    Item,
}

impl Default for EditTarget {
    fn default() -> Self {
        EditTarget::None
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel::List
    }
}

#[derive(Debug, Default)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
    pub sub_items: Vec<TodoItem>,
}