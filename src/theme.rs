use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    #[serde(skip)]
    pub background: Color,
    #[serde(skip)]
    pub foreground: Color,
    #[serde(skip)]
    pub accent: Color,
    #[serde(skip)]
    pub accent2: Color,
    #[serde(skip)]
    pub green: Color,
    #[serde(skip)]
    pub red: Color,
    #[serde(skip)]
    pub gray: Color,
    #[serde(skip)]
    pub selection_bg: Color,
}

impl Theme {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Catppuccin Mocha" => Self::catppuccin_mocha(),
            "Catppuccin Macchiato" => Self::catppuccin_macchiato(),
            "Catppuccin Frappé" => Self::catppuccin_frappe(),
            "Catppuccin Latte" => Self::catppuccin_latte(),
            "Gruvbox" => Self::gruvbox(),
            "Rosé Pine" => Self::rose_pine(),
            "Solarized Dark" => Self::solarized_dark(),
            _ => Self::catppuccin_mocha(),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            background: Color::Rgb(30, 30, 46),
            foreground: Color::Rgb(205, 214, 244),
            accent: Color::Rgb(203, 166, 247),
            accent2: Color::Rgb(116, 199, 236),
            green: Color::Rgb(166, 227, 161),
            red: Color::Rgb(243, 139, 168),
            gray: Color::Rgb(108, 112, 134),
            selection_bg: Color::Rgb(69, 71, 90),
        }
    }

    pub fn catppuccin_macchiato() -> Self {
        Self {
            name: "Catppuccin Macchiato".to_string(),
            background: Color::Rgb(36, 39, 58),
            foreground: Color::Rgb(202, 211, 245),
            accent: Color::Rgb(198, 160, 246),
            accent2: Color::Rgb(125, 196, 228),
            green: Color::Rgb(166, 218, 149),
            red: Color::Rgb(237, 135, 150),
            gray: Color::Rgb(110, 115, 141),
            selection_bg: Color::Rgb(91, 96, 120),
        }
    }

    pub fn catppuccin_frappe() -> Self {
        Self {
            name: "Catppuccin Frappé".to_string(),
            background: Color::Rgb(48, 52, 70),
            foreground: Color::Rgb(198, 208, 245),
            accent: Color::Rgb(202, 158, 230),
            accent2: Color::Rgb(133, 193, 220),
            green: Color::Rgb(166, 209, 137),
            red: Color::Rgb(231, 130, 132),
            gray: Color::Rgb(115, 121, 148),
            selection_bg: Color::Rgb(98, 104, 128),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            name: "Catppuccin Latte".to_string(),
            background: Color::Rgb(239, 241, 245),
            foreground: Color::Rgb(76, 79, 105),
            accent: Color::Rgb(136, 57, 239),
            accent2: Color::Rgb(32, 159, 181),
            green: Color::Rgb(64, 160, 43),
            red: Color::Rgb(210, 15, 57),
            gray: Color::Rgb(172, 176, 190),
            selection_bg: Color::Rgb(204, 208, 218),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "Gruvbox".to_string(),
            background: Color::Rgb(40, 40, 40),
            foreground: Color::Rgb(235, 219, 178),
            accent: Color::Rgb(215, 153, 33),
            accent2: Color::Rgb(131, 165, 152),
            green: Color::Rgb(152, 151, 26),
            red: Color::Rgb(204, 36, 29),
            gray: Color::Rgb(146, 131, 116),
            selection_bg: Color::Rgb(60, 56, 54),
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            name: "Rosé Pine".to_string(),
            background: Color::Rgb(25, 23, 36),
            foreground: Color::Rgb(224, 222, 244),
            accent: Color::Rgb(196, 167, 231),
            accent2: Color::Rgb(49, 116, 143),
            green: Color::Rgb(156, 207, 216),
            red: Color::Rgb(235, 111, 146),
            gray: Color::Rgb(110, 106, 134),
            selection_bg: Color::Rgb(42, 39, 63),
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            background: Color::Rgb(0, 43, 54),
            foreground: Color::Rgb(131, 148, 150),
            accent: Color::Rgb(181, 137, 0),
            accent2: Color::Rgb(38, 139, 210),
            green: Color::Rgb(133, 153, 0),
            red: Color::Rgb(220, 50, 47),
            gray: Color::Rgb(88, 110, 117),
            selection_bg: Color::Rgb(7, 54, 66),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}
