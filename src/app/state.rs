use crate::theme::Theme;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum FormAction {
    None,
    Submit,
    Escape,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppMode {
    Normal,
    EscMenu,
    ThemePicker,
    Keybinds,
    ConfirmExit,
    Search,
}

impl Default for AppMode {
    fn default() -> Self {
        AppMode::Normal
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Panel {
    List,
    NewList,
}

impl Default for Panel {
    fn default() -> Self {
        Panel::List
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ListType {
    List,
    Folder,
}

impl Default for ListType {
    fn default() -> Self {
        ListType::List
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ListGroupType {
    None,
    Separator,
}

impl Default for ListGroupType {
    fn default() -> Self {
        ListGroupType::None
    }
}


#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum ItemType {
    Task,
    Header,
    Separator,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Task
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub id: usize,
    pub name: String,
    pub items: Vec<TodoItem>,
    pub list_type: ListType,
    pub group: ListGroupType,
    pub parent_id: Option<usize>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub is_done: bool,
    pub description: String,
    pub sub_items: Vec<TodoItem>,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Appstate {
    pub items: Vec<TodoItem>,
    #[serde(skip)]
    pub list_state: ListState,
    pub is_add_new: bool,
    pub is_editing: bool,
    pub edit_target: EditTarget,
    #[serde(skip)]
    pub editing_sub_index: Option<usize>,
    #[serde(skip)]
    pub g_count: u8,
    #[serde(skip)]
    pub input_value: String,
    #[serde(skip)]
    pub cursor_position: usize,
    pub active_panel: Panel,
    pub lists: Vec<TodoList>,
    #[serde(skip)]
    pub lists_list_state: ListState,
    pub active_list_index: usize,
    pub current_list_index: usize,
    #[serde(skip)]
    pub confirming_delete: bool,
    #[serde(skip)]
    pub mode: AppMode,
    pub theme_name: String,
    #[serde(skip)]
    pub theme: Theme,
    #[serde(skip)]
    pub original_theme: Option<Theme>,
    #[serde(skip)]
    pub esc_menu_state: ListState,
    #[serde(skip)]
    pub theme_picker_state: ListState,
    #[serde(skip)]
    pub search_input_value: String,
    #[serde(skip)]
    pub search_cursor_position: usize,
    #[serde(skip)]
    pub show_completed: bool,
    #[serde(skip)]
    pub current_folder_id: Option<usize>,
}

impl Appstate {
    pub fn get_theme(&self) -> Theme {
        Theme::from_name(&self.theme_name)
    }
}

impl Default for Appstate {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            list_state: ListState::default(),
            is_add_new: false,
            is_editing: false,
            edit_target: EditTarget::default(),
            editing_sub_index: None,
            g_count: 0,
            input_value: String::new(),
            cursor_position: 0,
            active_panel: Panel::default(),
            lists: Vec::new(),
            lists_list_state: ListState::default(),
            active_list_index: 0,
            current_list_index: 0,
            confirming_delete: false,
            mode: AppMode::default(),
            theme_name: "Catppuccin Mocha".to_string(),
            theme: Theme::default(),
            original_theme: None,
            esc_menu_state: ListState::default(),
            theme_picker_state: ListState::default(),
            search_input_value: String::new(),
            search_cursor_position: 0,
            show_completed: true,
            current_folder_id: None,
        }
    }
}
