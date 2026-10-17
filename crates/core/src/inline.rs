//! Fountain inline markup: `*italic*`, `**bold**`, `***bold italic***`, `_underline_`,
//! `[[notes]]` and backslash escapes.
//!
//! Element text keeps the markup verbatim; [`parse`] turns it into styled runs for printing
//! and FDX, and [`write`] turns styled runs back into markup.

use serde::{Deserialize, Serialize};

/// Character styling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// A run of text with one style.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    pub text: String,
    #[serde(default)]
    pub style: Style,
    /// A `[[note]]`: kept in the document, not printed.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub note: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Marker {
    Italic,
    Bold,
    BoldItalic,
    Underline,
}

impl Marker {
    fn raw(self) -> &'static str {
        match self {
            Marker::Italic => "*",
            Marker::Bold => "**",
            Marker::BoldItalic => "***",
            Marker::Underline => "_",
        }
    }
}

enum Token {
    Text(String),
    Note(String),
    Marker(Marker),
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut text = String::new();
    let mut chars = source.chars().peekable();
    let flush = |text: &mut String, tokens: &mut Vec<Token>| {
        if !text.is_empty() {
            tokens.push(Token::Text(std::mem::take(text)));
        }
    };
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(next) => text.push(next),
                None => text.push('\\'),
            },
            '[' if chars.peek() == Some(&'[') => {
                chars.next();
                let mut note = String::new();
                let mut closed = false;
                while let Some(n) = chars.next() {
                    if n == ']' && chars.peek() == Some(&']') {
                        chars.next();
                        closed = true;
                        break;
                    }
                    note.push(n);
                }
                if closed {
                    flush(&mut text, &mut tokens);
                    tokens.push(Token::Note(note));
                } else {
                    text.push_str("[[");
                    text.push_str(&note);
                }
            }
            '*' => {
                let mut count = 1;
                while count < 3 && chars.peek() == Some(&'*') {
                    chars.next();
                    count += 1;
                }
                flush(&mut text, &mut tokens);
                tokens.push(Token::Marker(match count {
                    1 => Marker::Italic,
                    2 => Marker::Bold,
                    _ => Marker::BoldItalic,
                }));
            }
            '_' => {
                flush(&mut text, &mut tokens);
                tokens.push(Token::Marker(Marker::Underline));
            }
            other => text.push(other),
        }
    }
    flush(&mut text, &mut tokens);
    tokens
}

/// Parse markup into runs. Unmatched markers stay literal. Adjacent runs with the same style
/// are merged.
pub fn parse(source: &str) -> Vec<Run> {
    let tokens = tokenize(source);
    // Pair markers: a marker closes the most recent open marker of the same kind.
    let mut matched = vec![false; tokens.len()];
    let mut open: Vec<(usize, Marker)> = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if let Token::Marker(marker) = token {
            if let Some(position) = open.iter().rposition(|(_, kind)| kind == marker) {
                let (start, _) = open.remove(position);
                if let Some(slot) = matched.get_mut(start) {
                    *slot = true;
                }
                if let Some(slot) = matched.get_mut(index) {
                    *slot = true;
                }
            } else {
                open.push((index, *marker));
            }
        }
    }
    let mut runs: Vec<Run> = Vec::new();
    let mut style = Style::default();
    let mut active = [false; 4];
    let push = |runs: &mut Vec<Run>, text: &str, style: Style, note: bool| {
        if text.is_empty() {
            return;
        }
        if let Some(last) = runs.last_mut()
            && last.style == style
            && last.note == note
        {
            last.text.push_str(text);
            return;
        }
        runs.push(Run { text: text.to_string(), style, note });
    };
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Text(text) => push(&mut runs, text, style, false),
            Token::Note(text) => push(&mut runs, text, style, true),
            Token::Marker(marker) => {
                if !matched.get(index).copied().unwrap_or(false) {
                    push(&mut runs, marker.raw(), style, false);
                    continue;
                }
                let slot = match marker {
                    Marker::Italic => 0,
                    Marker::Bold => 1,
                    Marker::BoldItalic => 2,
                    Marker::Underline => 3,
                };
                if let Some(flag) = active.get_mut(slot) {
                    *flag = !*flag;
                }
                let [italic, bold, bold_italic, underline] = active;
                style = Style { bold: bold || bold_italic, italic: italic || bold_italic, underline };
            }
        }
    }
    runs
}

/// Plain printable text (styles and notes dropped).
pub fn plain(source: &str) -> String {
    parse(source).into_iter().filter(|run| !run.note).map(|run| run.text).collect()
}

fn escape(text: &str, out: &mut String) {
    for c in text.chars() {
        if matches!(c, '*' | '_' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
}

/// Write runs as markup. `parse(&write(runs))` returns the same runs when no two
/// differently-styled runs abut without a separating character of their own.
pub fn write(runs: &[Run]) -> String {
    let mut out = String::new();
    for run in runs {
        if run.note {
            out.push_str("[[");
            out.push_str(&run.text.replace("]]", "] ]"));
            out.push_str("]]");
            continue;
        }
        let star = match (run.style.bold, run.style.italic) {
            (true, true) => "***",
            (true, false) => "**",
            (false, true) => "*",
            (false, false) => "",
        };
        if run.style.underline {
            out.push('_');
        }
        out.push_str(star);
        let mut escaped = String::new();
        escape(&run.text, &mut escaped);
        out.push_str(&escaped);
        out.push_str(star);
        if run.style.underline {
            out.push('_');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn styled(text: &str, bold: bool, italic: bool, underline: bool) -> Run {
        Run { text: text.into(), style: Style { bold, italic, underline }, note: false }
    }

    #[test]
    fn parses_emphasis() {
        let runs = parse("a *b* **c** ***d*** _e_ f");
        assert_eq!(
            runs,
            vec![
                styled("a ", false, false, false),
                styled("b", false, true, false),
                styled(" ", false, false, false),
                styled("c", true, false, false),
                styled(" ", false, false, false),
                styled("d", true, true, false),
                styled(" ", false, false, false),
                styled("e", false, false, true),
                styled(" f", false, false, false),
            ]
        );
    }

    #[test]
    fn unmatched_markers_are_literal() {
        assert_eq!(plain("file_name and 5 * 3"), "file_name and 5 * 3");
        assert_eq!(plain(r"\*not italic\*"), "*not italic*");
    }

    #[test]
    fn notes_are_not_printed() {
        assert_eq!(plain("Hello [[fix this]]world"), "Hello world");
        assert_eq!(plain("Open [[ never closed"), "Open [[ never closed");
    }

    #[test]
    fn write_round_trips() {
        let runs = vec![styled("Plain * star ", false, false, false), styled("bold", true, false, true), styled(" end_", false, false, false)];
        assert_eq!(parse(&write(&runs)), runs);
    }
}
