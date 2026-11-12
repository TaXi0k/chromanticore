//! # ChroManticore
//! 
//! Inline text styling for terminal applications - parse markup like
//! `{[RED,BOLD]hello}` into ANSI-styled output.
//! 
//! ## Quick start
//! 
//! ```
//! use chromanticore::format_text;
//! 
//! let out = format_text("Hello, {[BOLD,RED]world}!").unwrap();
//! println!("{out}");
//! ```
//! 
//! Colors follow the user's terminal theme. To use a fixed palette,
//! pass a [`theme::Theme`] to [`themed_format_text`]:
//! 
//! ```
//! use chromanticore::{themed_format_text, theme};
//! 
//! let out = themed_format_text("Hello, {[PINK]world}!", &theme::MONOKAI).unwrap();
//! ```
//! 
//! ## Syntax
//! 
//! A styled block look like `{[STYLES]text}`:
//! 
//! - `STYLES` is a comma-separated list (spaces around commas are allowed) of style names (case-insensitive).
//! - Everything after `]` until the closing `}` is the styled text.
//! - Text outside `{...}` is left untouched.
//! 
//! ```
//! use chromanticore::format_text;
//!
//! format_text("plain {[BOLD]bold} plain").unwrap();
//! format_text("{[RED,GREEN]last color wins}").unwrap();
//! ```
//! 
//! ### Styles
//! 
//! **Attributes:** `BOLD`, `ITALIC`, `UNDERLINE` / `UNDER`, `STRIKETHROUGH` / `STRIKE`, `DIM`
//! 
//! **Colors:** `RED`, `ORANGE`, `YELLOW`, `GREEN`, `CYAN`, `SKY`, `BLUE`, `PURPLE`, `PINK`, `PALEGOLDENROD`, `REBECCAPURPLE`
//! 
//! ## Color output
//!
//! [`format_text`] emits 16-color ANSI escapes, so colors follow the
//! user's terminal palette. [`themed_format_text`] emits 24-bit truecolor
//! instead, so colors are fixed regardless of terminal settings.
//!
//! ## Themes
//!
//! [`themed_format_text`] takes a [`theme::Theme`]. See the [`theme`]
//! module for built-in presets and how to build your own.
//! 
//! ## Errors
//! 
//! Both functions return [`FormattingError`] if a styled block is malformed
//! (e.g. a missing `]` or an empty style list or no text to be styled).

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


use core::fmt;
use std::error::Error;

use owo_colors::{OwoColorize, Style};

// ========================================================================

pub mod theme;

// ========================================================================

/// Errors that can happen when parsing style markup.
#[derive(Debug, PartialEq)]
pub enum FormattingError {
    /// A style block (the part inside `{}`) could not be parsed.
    /// Contains the block that error happened on.
    WrongToken(String),
}

impl fmt::Display for FormattingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattingError::WrongToken(token) => {
                write!(f, "malformed style block {token}")
            }
        }
    }
}

impl Error for FormattingError {}

// ========================================================================

/// Parses inline style markup into truecolor ANSI-styled text.
/// 
/// Colors are taken from the provided [`Theme`](theme::Theme) and emitted
/// as 24-bit truecolor escapes, so the output does not depend on the
/// user's terminal palette. Use [`format_text`] if you want colors to
/// follow the terminal theme instead.
/// 
/// A [`Theme`](theme::Theme) is a plain struct with public fields - build
/// your own with struct update syntax, e.g. `Theme { pink: my_pink, ..theme::DEFAULT }`,
/// or start from one of the built-ins below.
/// 
/// See the [crate-level docs](crate) for the markup syntax.
/// 
/// ## Built-in themes
/// 
/// * [`theme::DEFAULT`]
/// * [`theme::CATPPUCCIN_LATTE`]
/// * [`theme::CATPPUCCIN_MOCHA`]
/// * [`theme::MONOKAI`]
/// 
/// ## Errors
/// 
/// Returns [`FormattingError`] if a style block is malformed - e.g. missing `]` or an empty style list.
pub fn themed_format_text(text: &str, theme: &theme::Theme) -> Result<String, FormattingError> {
    let mut tokens = tokenize(text);

    for token in &mut tokens {
        if Some('[') == token.chars().next() {
            
            match split_styles_and_text(&token) {
                Ok( (styles, text) ) => {
                    *token = parse_styles(&styles, &text, Some(theme));
                },
                Err(e) => return Err(e),
            }
            
        }
    }

    Ok( tokens.join("") )
}

/// Parses inline style markup into ANSI-styled text.
/// 
/// Colors follow the user's terminal theme (16-color ANSI), so the
/// output looks correct regardless of the terminal's configured palette.
/// Use [`themed_format_text`] if you want fixed colors.
/// 
/// See the [crate-level docs](crate) for the markup syntax.
/// 
/// ## Errors
/// 
/// Returns [`FormattingError`] if a style block is malformed - e.g. missing `]` or an empty style list.
pub fn format_text(text: &str) -> Result<String, FormattingError> {
    let mut tokens = tokenize(text);

    for token in &mut tokens {
        if Some('[') == token.chars().next() {
            
            match split_styles_and_text(&token) {
                Ok( (styles, text) ) => {
                    *token = parse_styles(&styles, &text, None);
                },
                Err(e) => return Err(e),
            }
            
        }
    }

    Ok( tokens.join("") )
}

// ========================================================================

fn tokenize(string: &str) -> Vec<String> {
    let tokens: Vec<String> = string.split(&[ '{', '}' ]).map(|t| t.to_owned()).collect();
    tokens
}


fn split_styles_and_text(token: &str) -> Result<(Vec<String>, String), FormattingError> {
    let parts: Vec<&str> = token.split(&[ '[', ']' ])
        .filter(|p| !p.is_empty())
        .collect();

    if parts.len() != 2 {
        return Err( FormattingError::WrongToken( format!("{{{}}}", token) ) );
    }

    let styles: Vec<String> = parts[0].split(",").map(|s| s.trim().to_string()).collect();
    let text = parts[1].to_string();

    Ok( (styles, text) )
}


fn parse_styles(styles: &[String], text: &str, theme_selection: Option<&theme::Theme>) -> String {
    let mut style = Style::new();

    // Styles
    if has_style(styles, "BOLD") { style = style.bold(); }
    if has_style(styles, "ITALIC") { style = style.italic(); }
    if has_style(styles, "UNDERLINE")|| has_style(styles, "UNDER") { style = style.underline(); }
    if has_style(styles, "STRIKETHROUGH") || has_style(styles, "STRIKE") { style = style.strikethrough(); }
    if has_style(styles, "DIM") { style = style.dimmed(); }

    // Colors
    match theme_selection {
        // Formatting with custom themes
        Some(color_theme) => {
            if has_style(styles, "RED") { style = style.truecolor(color_theme.red.red, color_theme.red.green, color_theme.red.blue); }
            if has_style(styles, "ORANGE") { style = style.truecolor(color_theme.orange.red, color_theme.orange.green, color_theme.orange.blue); }
            if has_style(styles, "YELLOW") { style = style.truecolor(color_theme.yellow.red, color_theme.yellow.green, color_theme.yellow.blue); }
            if has_style(styles, "GREEN") { style = style.truecolor(color_theme.green.red, color_theme.green.green, color_theme.green.blue); }
            if has_style(styles, "CYAN") { style = style.truecolor(color_theme.cyan.red, color_theme.cyan.green, color_theme.cyan.blue); }
            if has_style(styles, "SKY") { style = style.truecolor(color_theme.sky.red, color_theme.sky.green, color_theme.sky.blue); }
            if has_style(styles, "BLUE") { style = style.truecolor(color_theme.blue.red, color_theme.blue.green, color_theme.blue.blue); }
            if has_style(styles, "PURPLE") { style = style.truecolor(color_theme.purple.red, color_theme.purple.green, color_theme.purple.blue); }
            if has_style(styles, "PINK") { style = style.truecolor(color_theme.pink.red, color_theme.pink.green, color_theme.pink.blue); }
            if has_style(styles, "PALEGOLDENROD") { style = style.truecolor(color_theme.palegoldenrod.red, color_theme.palegoldenrod.green, color_theme.palegoldenrod.blue); }
            if has_style(styles, "REBECCAPURPLE") { style = style.truecolor(color_theme.rebeccapurple.red, color_theme.rebeccapurple.green, color_theme.rebeccapurple.blue); }
        },
        // Following users terminal theme
        None => {
            if has_style(styles, "RED") { style = style.red(); }
            if has_style(styles, "ORANGE") { style = style.yellow(); }
            if has_style(styles, "YELLOW") { style = style.bright_yellow(); }
            if has_style(styles, "GREEN") { style = style.green(); }
            if has_style(styles, "CYAN") { style = style.bright_green(); }
            if has_style(styles, "SKY") { style = style.bright_blue(); }
            if has_style(styles, "BLUE") { style = style.blue(); }
            if has_style(styles, "PURPLE") { style = style.magenta(); }
            if has_style(styles, "PINK") { style = style.bright_magenta(); }
            if has_style(styles, "PALEGOLDENROD") { style = style.truecolor(theme::DEFAULT.palegoldenrod.red, theme::DEFAULT.palegoldenrod.green, theme::DEFAULT.palegoldenrod.blue); }
            if has_style(styles, "REBECCAPURPLE") { style = style.truecolor(theme::DEFAULT.rebeccapurple.red, theme::DEFAULT.rebeccapurple.green, theme::DEFAULT.rebeccapurple.blue); }
        }
    }

    text.style(style).to_string()
}

fn has_style(styles: &[String], style: &str) -> bool {
    styles.iter().any(|s| s.eq_ignore_ascii_case(style))
}