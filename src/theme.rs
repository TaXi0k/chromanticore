//! Color themes for [`themed_format_text`](crate::themed_format_text).
//! 
//! A [`Theme`] maps each color name (`PINK`, `ORANGE`, `RED`, ...) to a
//! concrete [`Color`]. [`themed_format_text`](crate::themed_format_text)
//! resolves those names through the theme, so the markup renders
//! differently depending on which theme you pass.
//! 
//! # Built-in themes
//! 
//! | Const | Style |
//! | --- | --- |
//! | [`DEFAULT`] | X11 / CSS colors |
//! | [`CATPPUCCIN_LATTE`] | Catppuccin Latte, soft light theme. |
//! | [`CATPPUCCIN_MOCHA`] | Catppuccin Mocha, dark soothing pastel theme. |
//! | [`MONOKAI`] | Monokai, high-contrast vibrant dark theme. |

// ========================================================================

// Copyright 2026 TaXi0k
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ========================================================================

/// The default theme, following X11 / CSS colors.
/// 
/// Bright, fully saturated primaries - high contrast, but harsh on
/// long reading. Good as fallback, probably not best for daily driver.
pub const DEFAULT: Theme = Theme {
    pink:    Color { red: 255, green: 192, blue: 203 },
    red:     Color { red: 255, green:   0, blue:   0 },
    orange:  Color { red: 255, green: 165, blue:   0 },
    yellow:  Color { red: 255, green: 255, blue:   0 },
    green:   Color { red:   0, green: 255, blue:   0 },
    cyan:    Color { red:   0, green: 255, blue: 255 },
    sky:     Color { red: 135, green: 206, blue: 235 },
    blue:    Color { red:   0, green:   0, blue: 255 },
    purple:  Color { red: 160, green:  32, blue: 240 },

    palegoldenrod:  Color { red: 238, green: 232, blue: 170 },
    rebeccapurple: Color { red: 102, green:  51, blue: 153 },
};

/// [Catppuccin Latte](https://catppuccin.com/), soft light theme.
/// 
/// A clean, warm light palette optimized for daytime use, 
/// balancing high legibility with Catppuccin's signature gentle pastels.
pub const CATPPUCCIN_LATTE: Theme = Theme {
    pink:    Color { red: 234, green: 118, blue: 203 },
    red:     Color { red: 210, green:  15, blue:  57 },
    orange:  Color { red: 254, green: 100, blue:  11 },
    yellow:  Color { red: 223, green: 142, blue:  29 },
    green:   Color { red:  64, green: 160, blue:  43 },
    cyan:    Color { red:  23, green: 146, blue: 153 },
    sky:     Color { red:   4, green: 165, blue: 229 },
    blue:    Color { red:  30, green: 102, blue: 245 },
    purple:  Color { red: 136, green:  57, blue: 239 },

    palegoldenrod:  Color { red: 238, green: 232, blue: 170 },
    rebeccapurple: Color { red: 102, green:  51, blue: 153 },
};

/// [Catppuccin Mocha](https://catppuccin.com/), dark soothing pastel theme.
/// 
/// A popular dark palette designed to be easy on the eyes during 
/// extended coding or reading sessions, with soft, muted pastel tones.
pub const CATPPUCCIN_MOCHA: Theme = Theme {
    pink:    Color { red: 245, green: 194, blue: 231 },
    red:     Color { red: 243, green: 139, blue: 168 },
    orange:  Color { red: 250, green: 179, blue: 135 },
    yellow:  Color { red: 249, green: 226, blue: 175 },
    green:   Color { red: 166, green: 227, blue: 161 },
    cyan:    Color { red: 148, green: 226, blue: 213 },
    sky:     Color { red: 137, green: 220, blue: 235 },
    blue:    Color { red: 137, green: 180, blue: 250 },
    purple:  Color { red: 203, green: 166, blue: 247 },

    palegoldenrod:  Color { red: 238, green: 232, blue: 170 },
    rebeccapurple: Color { red: 102, green:  51, blue: 153 },
};

/// [Monokai](https://monokai.pro/), high-contrast vibrant dark theme.
/// 
/// Inspired by the classic text editor color scheme, featuring vivid 
/// neon greens, pinks, and yellows on dark backgrounds. Great for readability 
/// and modern aesthetic.
pub const MONOKAI: Theme = Theme {
    pink:    Color { red: 255, green: 121, blue: 198 },
    red:     Color { red: 249, green:  38, blue: 114 },
    orange:  Color { red: 253, green: 151, blue:  31 },
    yellow:  Color { red: 230, green: 219, blue: 116 },
    green:   Color { red: 166, green: 226, blue:  46 },
    cyan:    Color { red: 102, green: 217, blue: 239 },
    sky:     Color { red: 139, green: 233, blue: 253 }, 
    blue:    Color { red: 137, green: 180, blue: 250 }, 
    purple:  Color { red: 174, green: 129, blue: 255 },

    palegoldenrod:  Color { red: 238, green: 232, blue: 170 },
    rebeccapurple: Color { red: 102, green:  51, blue: 153 },
};

// ========================================================================

/// A palette of named colors used by [`themed_format_text`](crate::themed_format_text).
///
/// All fields are public, so you can build a custom theme with struct
/// update syntax:
///
/// ```
/// use chromanticore::theme::{self, Theme, Color};
///
/// let my_theme = Theme {
///     pink: Color { red: 255, green: 0, blue: 128 },
///     ..theme::DEFAULT
/// };
/// ```
///
/// Building a theme by listing every field (without `..`) works too, but
/// it will stop compiling if a new color is added in a future version.
/// Prefer `..theme::DEFAULT` (or another preset) as the base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme {
    pub pink: Color,
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub cyan: Color,
    pub sky: Color,
    pub blue: Color,
    pub purple: Color,
    
    pub palegoldenrod: Color,
    pub rebeccapurple: Color,
}
impl Theme {
    pub fn new() -> Self {
        todo!()
    }
}

/// RGB color, one byte per channel. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}