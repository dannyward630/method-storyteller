//! Autocomplete for character cues, scene headings and transitions.
//!
//! Suggestions are complete replacement texts for the element, best first.

use std::collections::HashMap;

use crate::model::{ElementKind, Script, character_name};

/// Most suggestions returned.
pub const MAX_SUGGESTIONS: usize = 10;

pub const SCENE_PREFIXES: [&str; 5] = ["INT.", "EXT.", "INT./EXT.", "EXT./INT.", "EST."];
pub const TIMES_OF_DAY: [&str; 9] = ["DAY", "NIGHT", "MORNING", "AFTERNOON", "EVENING", "CONTINUOUS", "LATER", "MOMENTS LATER", "SAME"];
pub const TRANSITIONS: [&str; 9] =
    ["CUT TO:", "FADE IN:", "FADE OUT.", "FADE TO BLACK.", "DISSOLVE TO:", "SMASH CUT TO:", "MATCH CUT TO:", "JUMP CUT TO:", "BACK TO:"];
pub const EXTENSIONS: [&str; 4] = ["V.O.", "O.S.", "O.C.", "CONT'D"];

/// Ranked, de-duplicated candidates starting with `prefix` (but not equal to it).
fn rank(prefix: &str, candidates: impl IntoIterator<Item = (String, usize)>) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for (candidate, weight) in candidates {
        if candidate.starts_with(prefix) && candidate != prefix && !candidate.is_empty() {
            *counts.entry(candidate).or_default() += weight;
        }
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.into_iter().take(MAX_SUGGESTIONS).map(|(text, _)| text).collect()
}

/// Split a heading into (prefix, location, time of day).
pub fn split_heading(text: &str) -> (Option<&str>, &str, Option<&str>) {
    let upper_starts = |p: &str| text.get(..p.len()).is_some_and(|head| head.eq_ignore_ascii_case(p));
    // Longest prefix first so `INT./EXT.` wins over `INT.`.
    let mut prefixes = SCENE_PREFIXES;
    prefixes.sort_by_key(|p| std::cmp::Reverse(p.len()));
    let (prefix, rest) = match prefixes.iter().find(|p| upper_starts(p)) {
        Some(p) => (text.get(..p.len()), text.get(p.len()..).unwrap_or("")),
        None => (None, text),
    };
    let rest = rest.trim_start();
    match rest.rsplit_once(" - ") {
        Some((location, time)) => (prefix, location.trim(), Some(time.trim())),
        None => (prefix, rest.trim_end(), None),
    }
}

/// Suggestions for an element of `kind` whose current text is `text`.
pub fn suggest(script: &Script, kind: ElementKind, text: &str) -> Vec<String> {
    let typed = text.to_uppercase();
    if typed.trim().is_empty() {
        return Vec::new();
    }
    match kind {
        ElementKind::Character => {
            if let Some(open) = typed.rfind('(') {
                let base = typed.get(..open).unwrap_or("").trim_end();
                let partial = typed.get(open + 1..).unwrap_or("");
                return rank(partial, EXTENSIONS.iter().map(|e| (e.to_string(), 1))).into_iter().map(|e| format!("{base} ({e})")).collect();
            }
            let names = script.elements.iter().filter(|e| e.kind == ElementKind::Character).map(|e| (character_name(&e.text).to_uppercase(), 1));
            rank(&typed, names)
        }
        ElementKind::SceneHeading => {
            let (prefix, location, time) = split_heading(&typed);
            let headings: Vec<String> =
                script.elements.iter().filter(|e| e.kind == ElementKind::SceneHeading).map(|e| e.text.to_uppercase()).collect();
            match (prefix, time) {
                (None, _) => rank(&typed, SCENE_PREFIXES.iter().map(|p| (p.to_string(), 1))),
                (Some(prefix), None) => {
                    let locations = headings.iter().filter_map(|h| {
                        let (_, location, _) = split_heading(h);
                        (!location.is_empty()).then(|| (location.to_string(), 1))
                    });
                    rank(location, locations).into_iter().map(|l| format!("{prefix} {l}")).collect()
                }
                (Some(prefix), Some(time)) => {
                    let used = headings.iter().filter_map(|h| split_heading(h).2.map(|t| (t.to_string(), 2)));
                    let builtin = TIMES_OF_DAY.iter().map(|t| (t.to_string(), 1));
                    rank(time, used.chain(builtin)).into_iter().map(|t| format!("{prefix} {location} - {t}")).collect()
                }
            }
        }
        ElementKind::Transition => {
            let used = script.elements.iter().filter(|e| e.kind == ElementKind::Transition).map(|e| (e.text.to_uppercase(), 2));
            rank(typed.trim_start(), used.chain(TRANSITIONS.iter().map(|t| (t.to_string(), 1))))
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Element;

    fn script() -> Script {
        Script {
            elements: vec![
                Element::new(ElementKind::SceneHeading, "INT. KITCHEN - NIGHT"),
                Element::new(ElementKind::Character, "MARGARET"),
                Element::new(ElementKind::Dialogue, "Hi."),
                Element::new(ElementKind::Character, "MARGARET (V.O.)"),
                Element::new(ElementKind::Dialogue, "Again."),
                Element::new(ElementKind::Character, "MARCUS"),
                Element::new(ElementKind::Dialogue, "Yes."),
                Element::new(ElementKind::SceneHeading, "EXT. KITCHEN GARDEN - DAY"),
            ],
            ..Script::default()
        }
    }

    #[test]
    fn characters_by_frequency() {
        assert_eq!(suggest(&script(), ElementKind::Character, "mar"), ["MARGARET", "MARCUS"]);
        assert_eq!(suggest(&script(), ElementKind::Character, "MARCUS (v"), ["MARCUS (V.O.)"]);
        assert!(suggest(&script(), ElementKind::Character, "").is_empty());
    }

    #[test]
    fn scene_heading_parts() {
        assert_eq!(suggest(&script(), ElementKind::SceneHeading, "i"), ["INT.", "INT./EXT."]);
        assert_eq!(suggest(&script(), ElementKind::SceneHeading, "int. kit"), ["INT. KITCHEN", "INT. KITCHEN GARDEN"]);
        assert_eq!(suggest(&script(), ElementKind::SceneHeading, "INT. HALL - N")[0], "INT. HALL - NIGHT");
    }

    #[test]
    fn transitions() {
        assert_eq!(suggest(&script(), ElementKind::Transition, "sm"), ["SMASH CUT TO:"]);
    }
}
