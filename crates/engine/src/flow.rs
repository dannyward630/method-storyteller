//! Final Draft–style element flow (`docs/spec.md` §4.1), as data.

use storyteller_core::ElementKind as K;

/// One row of the flow table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowRule {
    pub kind: K,
    /// Element started by Enter at the end of a non-empty element.
    pub enter: K,
    /// What Enter turns an empty element into.
    pub enter_empty: K,
    pub tab: K,
    pub shift_tab: K,
}

const fn rule(kind: K, enter: K, enter_empty: K, tab: K, shift_tab: K) -> FlowRule {
    FlowRule { kind, enter, enter_empty, tab, shift_tab }
}

/// The default flow table.
pub const RULES: [FlowRule; 7] = [
    rule(K::SceneHeading, K::Action, K::Action, K::Action, K::Action),
    rule(K::Action, K::Action, K::SceneHeading, K::Character, K::SceneHeading),
    rule(K::Character, K::Dialogue, K::Action, K::Action, K::Action),
    rule(K::Parenthetical, K::Dialogue, K::Dialogue, K::Dialogue, K::Dialogue),
    rule(K::Dialogue, K::Character, K::Action, K::Parenthetical, K::Character),
    rule(K::Transition, K::SceneHeading, K::SceneHeading, K::SceneHeading, K::Action),
    rule(K::Shot, K::Action, K::Action, K::Action, K::Action),
];

/// The rule for a kind; kinds without one behave like action.
pub fn rule_for(kind: K) -> FlowRule {
    RULES.iter().copied().find(|r| r.kind == kind).unwrap_or(FlowRule {
        kind,
        enter: K::Action,
        enter_empty: K::Action,
        tab: K::Character,
        shift_tab: K::SceneHeading,
    })
}

/// Element types reachable by ⌘1–⌘7.
pub const SHORTCUT_KINDS: [K; 7] = [K::SceneHeading, K::Action, K::Character, K::Parenthetical, K::Dialogue, K::Transition, K::Shot];

/// Re-shape text when an element changes kind: parentheticals gain or lose their brackets.
/// Returns the new text and how far the caret moves (in bytes, may be negative).
pub fn convert_text(text: &str, from: K, to: K) -> (String, isize) {
    if from == to {
        return (text.to_string(), 0);
    }
    if to == K::Parenthetical {
        let trimmed = text.trim();
        if trimmed.starts_with('(') {
            return (text.to_string(), 0);
        }
        return (format!("({trimmed})"), 1);
    }
    if from == K::Parenthetical {
        let trimmed = text.trim();
        if let Some(inner) = trimmed.strip_prefix('(') {
            let inner = inner.strip_suffix(')').unwrap_or(inner);
            return (inner.to_string(), -1);
        }
    }
    let text = if to.is_caps() { text.to_uppercase() } else { text.to_string() };
    (text, 0)
}
