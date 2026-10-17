//! `cargo xtask corpus`: measures import and pagination against private Final Draft files.
//!
//! The files are the maintainer's own scripts (and possibly other people's), so they never
//! enter the repository and nothing about their content is printed. `STORYTELLER_CORPUS`
//! names a manifest outside the work tree: one `.fdx` path per line, optionally followed by a
//! tab and the `.pdf` Final Draft exported from it. `#` starts a comment.
//!
//! For each file it checks that a second import of our export is identical, then compares
//! each scene's start page with the page Final Draft recorded in the file
//! (`SceneProperties@Page`), and the page count with the PDF's. Output is numbers only:
//! anonymous file numbers, counts, percentages and page numbers.

use std::path::{Path, PathBuf};
use std::process::Command;

use storyteller_core::layout::Layout;
use storyteller_core::model::ElementKind;
use storyteller_core::{fdx, paginate};

/// Corpus results for one file.
#[derive(Default)]
struct FileResult {
    scenes: usize,
    exact: usize,
    max_drift: u32,
    first_miss: Option<(usize, u32, u32)>,
    pages: usize,
    pdf_pages: Option<usize>,
    round_trip: bool,
}

fn inside(path: &Path, root: &Path) -> bool {
    match (path.canonicalize(), root.canonicalize()) {
        (Ok(path), Ok(root)) => path.starts_with(root),
        _ => false,
    }
}

/// The leading number of a page label such as `12` or `12A`.
fn page_number(label: &str) -> Option<u32> {
    let digits: String = label.trim().chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// The page Final Draft recorded for a scene heading, from its kept `SceneProperties`.
fn cached_page(children: &str) -> Option<u32> {
    let properties = children.find("<SceneProperties").map(|at| children.get(at..).unwrap_or_default())?;
    let tag = properties.get(..properties.find('>')?)?;
    let value = tag.split(" Page=\"").nth(1)?.split('"').next()?;
    page_number(value)
}

fn pdf_pages(pdf: &Path) -> Option<usize> {
    let output = Command::new("pdfinfo").arg(pdf).output().ok()?;
    String::from_utf8_lossy(&output.stdout).lines().find_map(|l| l.strip_prefix("Pages:").and_then(|n| n.trim().parse().ok()))
}

fn measure(fdx_path: &Path, pdf: Option<&Path>) -> Result<FileResult, String> {
    let bytes = std::fs::read(fdx_path).map_err(|e| format!("unreadable ({})", e.kind()))?;
    let text = storyteller_core::decode_text(&bytes).map_err(|_| "not UTF-8 text".to_string())?;
    let script = fdx::parse(text).map_err(|_| "does not import".to_string())?;
    let mut result = FileResult { round_trip: fdx::parse(&fdx::write(&script)).is_ok_and(|again| again == script), ..FileResult::default() };

    let paginated = paginate::paginate(&script, &Layout::for_script(&script));
    result.pages = paginated.page_count() + usize::from(paginated.title_page.is_some());
    for (index, scene) in paginated.scenes.iter().enumerate() {
        let Some(element) = script.elements.get(scene.element) else { continue };
        if element.kind != ElementKind::SceneHeading {
            continue;
        }
        let Some(expected) = element.fdx.as_ref().and_then(|f| cached_page(&f.children)) else { continue };
        result.scenes += 1;
        let drift = scene.page.abs_diff(expected);
        if drift == 0 {
            result.exact += 1;
        } else if result.first_miss.is_none() {
            result.first_miss = Some((index + 1, expected, scene.page));
        }
        result.max_drift = result.max_drift.max(drift);
    }
    result.pdf_pages = pdf.and_then(pdf_pages);
    Ok(result)
}

fn percent(part: usize, whole: usize) -> String {
    if whole == 0 { "-".into() } else { format!("{:.1}%", part as f64 * 100.0 / whole as f64) }
}

pub fn run(root: &Path) -> Result<(), String> {
    let manifest = std::env::var_os("STORYTELLER_CORPUS")
        .map(PathBuf::from)
        .ok_or("set STORYTELLER_CORPUS to a manifest outside the repository (see xtask/src/corpus.rs)")?;
    if inside(&manifest, root) {
        return Err("the corpus manifest must live outside the repository".into());
    }
    let list = std::fs::read_to_string(&manifest).map_err(|e| format!("cannot read the manifest: {}", e.kind()))?;
    let entries: Vec<(PathBuf, Option<PathBuf>)> = list
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let mut parts = l.split('\t');
            (PathBuf::from(parts.next().unwrap_or_default()), parts.next().map(PathBuf::from))
        })
        .collect();

    let (mut files, mut failed, mut round_trips, mut scenes, mut exact, mut all_exact_files, mut page_matches, mut with_pdf) =
        (0, 0, 0, 0, 0, 0, 0, 0);
    let mut max_drift = 0;
    for (n, (fdx_path, pdf)) in entries.iter().enumerate() {
        let n = n + 1;
        if inside(fdx_path, root) || pdf.as_deref().is_some_and(|p| inside(p, root)) {
            return Err(format!("file {n} is inside the repository; corpus files must stay outside it"));
        }
        files += 1;
        match measure(fdx_path, pdf.as_deref()) {
            Err(why) => {
                failed += 1;
                println!("file {n:>3}: {why}");
            }
            Ok(r) => {
                round_trips += usize::from(r.round_trip);
                scenes += r.scenes;
                exact += r.exact;
                all_exact_files += usize::from(r.scenes > 0 && r.exact == r.scenes);
                max_drift = max_drift.max(r.max_drift);
                let pdf_note = match r.pdf_pages {
                    Some(fd) => {
                        with_pdf += 1;
                        page_matches += usize::from(fd == r.pages);
                        format!(", pages {}/{fd}", r.pages)
                    }
                    None => format!(", pages {}", r.pages),
                };
                let miss = r.first_miss.map(|(s, fd, ours)| format!(", first miss scene {s} (FD p.{fd}, ours p.{ours})")).unwrap_or_default();
                println!(
                    "file {n:>3}: scenes {:>3}, exact {:>6}, max drift {}{pdf_note}{miss}{}",
                    r.scenes,
                    percent(r.exact, r.scenes),
                    r.max_drift,
                    if r.round_trip { "" } else { ", ROUND TRIP DIFFERS" }
                );
            }
        }
    }
    println!(
        "corpus: {files} files, {failed} failed to import, {round_trips} round-trip identically; scenes exact {exact}/{scenes} ({}), files scene-exact {all_exact_files}, max drift {max_drift} pages; page count matches PDF {page_matches}/{with_pdf}",
        percent(exact, scenes)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_cached_page() {
        assert_eq!(cached_page(r#"<SceneProperties Length="2/8" Page="12A" Title=""/>"#), Some(12));
        assert_eq!(cached_page(r#"<SceneProperties Length="2/8" Title=""/>"#), None);
        assert_eq!(cached_page("<OmittedScene/>"), None);
    }
}
