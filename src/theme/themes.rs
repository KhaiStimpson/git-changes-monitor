use ratatui::style::Color;

/// Application theme with semantic colors
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Display name for the theme
    pub display_name: String,

    // Base colors
    /// Main background color
    pub base: Color,
    /// Surface/section background color
    pub surface: Color,
    /// Overlay/popup background color
    pub overlay: Color,

    // Text colors
    /// Primary text color
    pub text: Color,
    /// Secondary/dimmed text color
    pub subtext: Color,

    // Accent colors
    /// Primary accent color
    pub accent: Color,
    /// Secondary accent color
    pub accent_secondary: Color,

    // Semantic colors
    /// Success/added color (green)
    pub success: Color,
    /// Error/deleted color (red)
    pub error: Color,
    /// Warning/modified color (yellow)
    pub warning: Color,
    /// Info color (blue)
    pub info: Color,

    // Git-specific colors
    /// Staged files indicator color
    pub staged: Color,
    /// Unstaged files indicator color
    pub unstaged: Color,
    /// Untracked files color
    pub untracked: Color,

    // Selection colors
    /// Selection background color
    pub selection: Color,
    /// Selection text color
    pub selection_text: Color,

    // Border color
    pub border: Color,
}

impl Theme {
    /// Get a theme by name
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "nord" => Self::nord(),
            "tokyo-night" | "tokyonight" => Self::tokyo_night(),
            "dracula" => Self::dracula(),
            _ => Self::catppuccin(), // Default to catppuccin
        }
    }

    /// Catppuccin Mocha theme
    pub fn catppuccin() -> Self {
        Self {
            name: "catppuccin".to_string(),
            display_name: "Catppuccin Mocha".to_string(),

            // Base colors
            base: Color::Rgb(30, 30, 46),    // #1e1e2e
            surface: Color::Rgb(49, 50, 68), // #313244
            overlay: Color::Rgb(69, 71, 90), // #45475a

            // Text colors
            text: Color::Rgb(205, 214, 244),    // #cdd6f4
            subtext: Color::Rgb(166, 173, 200), // #a6adc8

            // Accent colors
            accent: Color::Rgb(203, 166, 247), // #cba6f7 (mauve)
            accent_secondary: Color::Rgb(245, 194, 231), // #f5c2e7 (pink)

            // Semantic colors
            success: Color::Rgb(166, 227, 161), // #a6e3a1 (green)
            error: Color::Rgb(243, 139, 168),   // #f38ba8 (red)
            warning: Color::Rgb(249, 226, 175), // #f9e2af (yellow)
            info: Color::Rgb(137, 180, 250),    // #89b4fa (blue)

            // Git-specific
            staged: Color::Rgb(166, 227, 161),    // #a6e3a1 (green)
            unstaged: Color::Rgb(249, 226, 175),  // #f9e2af (yellow)
            untracked: Color::Rgb(147, 153, 178), // #9399b2 (overlay2)

            // Selection
            selection: Color::Rgb(69, 71, 90),         // #45475a
            selection_text: Color::Rgb(205, 214, 244), // #cdd6f4

            // Border
            border: Color::Rgb(88, 91, 112), // #585b70 (surface2)
        }
    }

    /// Nord theme
    pub fn nord() -> Self {
        Self {
            name: "nord".to_string(),
            display_name: "Nord".to_string(),

            // Base colors
            base: Color::Rgb(46, 52, 64),    // #2e3440 (nord0)
            surface: Color::Rgb(59, 66, 82), // #3b4252 (nord1)
            overlay: Color::Rgb(67, 76, 94), // #434c5e (nord2)

            // Text colors
            text: Color::Rgb(236, 239, 244),    // #eceff4 (nord6)
            subtext: Color::Rgb(216, 222, 233), // #d8dee9 (nord4)

            // Accent colors
            accent: Color::Rgb(136, 192, 208), // #88c0d0 (nord8 - frost)
            accent_secondary: Color::Rgb(129, 161, 193), // #81a1c1 (nord9)

            // Semantic colors
            success: Color::Rgb(163, 190, 140), // #a3be8c (nord14 - green)
            error: Color::Rgb(191, 97, 106),    // #bf616a (nord11 - red)
            warning: Color::Rgb(235, 203, 139), // #ebcb8b (nord13 - yellow)
            info: Color::Rgb(129, 161, 193),    // #81a1c1 (nord9 - blue)

            // Git-specific
            staged: Color::Rgb(163, 190, 140),   // #a3be8c (green)
            unstaged: Color::Rgb(235, 203, 139), // #ebcb8b (yellow)
            untracked: Color::Rgb(76, 86, 106),  // #4c566a (nord3)

            // Selection
            selection: Color::Rgb(67, 76, 94), // #434c5e (nord2)
            selection_text: Color::Rgb(236, 239, 244), // #eceff4

            // Border
            border: Color::Rgb(76, 86, 106), // #4c566a (nord3)
        }
    }

    /// Tokyo Night theme
    pub fn tokyo_night() -> Self {
        Self {
            name: "tokyo-night".to_string(),
            display_name: "Tokyo Night".to_string(),

            // Base colors
            base: Color::Rgb(26, 27, 38),     // #1a1b26
            surface: Color::Rgb(36, 40, 59),  // #24283b
            overlay: Color::Rgb(65, 72, 104), // #414868

            // Text colors
            text: Color::Rgb(169, 177, 214),  // #a9b1d6
            subtext: Color::Rgb(86, 95, 137), // #565f89

            // Accent colors
            accent: Color::Rgb(187, 154, 247), // #bb9af7 (purple)
            accent_secondary: Color::Rgb(255, 0, 127), // #ff007c (magenta)

            // Semantic colors
            success: Color::Rgb(158, 206, 106), // #9ece6a (green)
            error: Color::Rgb(247, 118, 142),   // #f7768e (red)
            warning: Color::Rgb(224, 175, 104), // #e0af68 (yellow)
            info: Color::Rgb(125, 207, 255),    // #7dcfff (cyan)

            // Git-specific
            staged: Color::Rgb(158, 206, 106),   // #9ece6a (green)
            unstaged: Color::Rgb(224, 175, 104), // #e0af68 (yellow)
            untracked: Color::Rgb(86, 95, 137),  // #565f89

            // Selection
            selection: Color::Rgb(65, 72, 104),        // #414868
            selection_text: Color::Rgb(169, 177, 214), // #a9b1d6

            // Border
            border: Color::Rgb(41, 46, 66), // #292e42
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            display_name: "Dracula".to_string(),

            // Base colors
            base: Color::Rgb(40, 42, 54),      // #282a36
            surface: Color::Rgb(68, 71, 90),   // #44475a
            overlay: Color::Rgb(98, 114, 164), // #6272a4

            // Text colors
            text: Color::Rgb(248, 248, 242),   // #f8f8f2
            subtext: Color::Rgb(98, 114, 164), // #6272a4

            // Accent colors
            accent: Color::Rgb(189, 147, 249), // #bd93f9 (purple)
            accent_secondary: Color::Rgb(255, 121, 198), // #ff79c6 (pink)

            // Semantic colors
            success: Color::Rgb(80, 250, 123),  // #50fa7b (green)
            error: Color::Rgb(255, 85, 85),     // #ff5555 (red)
            warning: Color::Rgb(241, 250, 140), // #f1fa8c (yellow)
            info: Color::Rgb(139, 233, 253),    // #8be9fd (cyan)

            // Git-specific
            staged: Color::Rgb(80, 250, 123),    // #50fa7b (green)
            unstaged: Color::Rgb(241, 250, 140), // #f1fa8c (yellow)
            untracked: Color::Rgb(98, 114, 164), // #6272a4

            // Selection
            selection: Color::Rgb(68, 71, 90),         // #44475a
            selection_text: Color::Rgb(248, 248, 242), // #f8f8f2

            // Border
            border: Color::Rgb(98, 114, 164), // #6272a4
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin()
    }
}
