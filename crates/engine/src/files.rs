//! Reading and writing script files. Saves are atomic (`craft-atomic`): a failed save never
//! damages the previous file.

use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};
use storyteller_core::layout::Layout;
use storyteller_core::{CoreError, MAX_INPUT_BYTES, Script, decode_text, fdx, fountain};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileFormat {
    #[default]
    Fountain,
    Fdx,
    Pdf,
}

impl FileFormat {
    /// Format implied by a file extension (Fountain for anything unknown).
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
            Some("fdx") => FileFormat::Fdx,
            Some("pdf") => FileFormat::Pdf,
            _ => FileFormat::Fountain,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            FileFormat::Fountain => "fountain",
            FileFormat::Fdx => "fdx",
            FileFormat::Pdf => "pdf",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("could not read {path}: {source}")]
    Read { path: String, source: std::io::Error },
    #[error("could not save {path}: {source}")]
    Write { path: String, source: std::io::Error },
    #[error("{0}")]
    Format(#[from] CoreError),
    #[error("PDF export failed: {0}")]
    Pdf(#[from] storyteller_pdf::PdfError),
    #[error("scripts cannot be opened from PDF")]
    CannotOpenPdf,
}

/// Read and parse a script; the format is sniffed from the content, then the extension.
pub fn open(path: &Path) -> Result<(Script, FileFormat), FileError> {
    let read_error = |source| FileError::Read { path: path.display().to_string(), source };
    let file = std::fs::File::open(path).map_err(read_error)?;
    let mut bytes = Vec::new();
    // Read one byte past the limit so oversize files are refused rather than truncated.
    file.take(MAX_INPUT_BYTES as u64 + 1).read_to_end(&mut bytes).map_err(read_error)?;
    let text = decode_text(&bytes)?;
    let head = text.trim_start();
    let format = if head.starts_with("<?xml") || head.starts_with("<FinalDraft") {
        FileFormat::Fdx
    } else {
        match FileFormat::from_path(path) {
            FileFormat::Pdf => return Err(FileError::CannotOpenPdf),
            other => other,
        }
    };
    let script = match format {
        FileFormat::Fdx => fdx::parse(text)?,
        _ => fountain::parse(text)?,
    };
    Ok((script, format))
}

/// Serialize a script in a format.
pub fn encode(script: &Script, format: FileFormat, layout: &Layout) -> Result<Vec<u8>, FileError> {
    Ok(match format {
        FileFormat::Fountain => fountain::write(script).into_bytes(),
        FileFormat::Fdx => fdx::write_with_layout(script, layout).into_bytes(),
        FileFormat::Pdf => storyteller_pdf::export(script, layout, &storyteller_pdf::PdfOptions { creator: crate::app_version() })?,
    })
}

/// Atomically write a script in a format.
pub fn save(script: &Script, path: &Path, format: FileFormat, layout: &Layout) -> Result<(), FileError> {
    let bytes = encode(script, format, layout)?;
    craft_atomic::atomic_write(path, &bytes).map_err(|source| FileError::Write { path: path.display().to_string(), source })
}
