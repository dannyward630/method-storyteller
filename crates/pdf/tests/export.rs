#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use storyteller_core::fountain;
use storyteller_core::layout::Layout;
use storyteller_core::model::{Element, ElementKind, Script};
use storyteller_pdf::{PdfOptions, export};

fn fixture() -> Script {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../core/tests/fixtures/lighthouse.fountain")).unwrap();
    fountain::parse(&text).unwrap()
}

fn count(haystack: &[u8], needle: &[u8]) -> usize {
    haystack.windows(needle.len()).filter(|window| *window == needle).count()
}

#[test]
fn exports_a_valid_pdf() {
    let bytes = export(&fixture(), &Layout::default(), &PdfOptions { creator: "Method Storyteller test".into() }).unwrap();
    assert!(bytes.starts_with(b"%PDF-"));
    assert!(bytes.trim_ascii_end().ends_with(b"%%EOF"));
    assert!(count(&bytes, b"/Outlines") >= 1, "scene bookmarks");
    assert!(count(&bytes, b"CourierPrime") >= 1, "font embedded");
}

#[test]
fn empty_and_odd_scripts_export() {
    for script in [
        Script::default(),
        Script { elements: vec![Element::new(ElementKind::Action, "日本語 and emoji 🎬 and **bold _under_**")], ..Script::default() },
    ] {
        let bytes = export(&script, &Layout::default(), &PdfOptions::default()).unwrap();
        assert!(bytes.starts_with(b"%PDF-"));
    }
}

/// Optional check with poppler's `pdftotext` when it is installed.
#[test]
fn text_is_extractable_when_pdftotext_is_available() {
    let Ok(output) = std::process::Command::new("pdftotext").arg("-v").output() else { return };
    if !output.status.success() && output.stderr.is_empty() {
        return;
    }
    let dir = std::env::temp_dir().join(format!("storyteller-pdf-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("lighthouse.pdf");
    std::fs::write(&path, export(&fixture(), &Layout::default(), &PdfOptions::default()).unwrap()).unwrap();
    let text = std::process::Command::new("pdftotext").arg("-layout").arg(&path).arg("-").output().unwrap();
    let text = String::from_utf8_lossy(&text.stdout);
    assert!(text.contains("EXT. LIGHTHOUSE - NIGHT"), "{text}");
    assert!(text.contains("OWEN (CONT'D)"), "{text}");
    if let Ok(info) = std::process::Command::new("pdfinfo").arg(&path).output() {
        let info = String::from_utf8_lossy(&info.stdout);
        // Title page plus two script pages.
        assert!(info.lines().any(|l| l.starts_with("Pages:") && l.trim_end().ends_with(" 3")), "{info}");
    }
    let _ = std::fs::remove_dir_all(dir);
}
