use std::{
    char::decode_utf16,
    io::{Cursor, Error as IoError, Read},
    path::Path,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;
use serde::Deserialize;
use zip::ZipArchive;

pub struct Report {
    pub theme: String,
    pub pages: Vec<Page>,
}

impl From<RawReport> for Report {
    fn from(value: RawReport) -> Self {
        Self {
            theme: value.theme,
            pages: {
                #[cfg(feature = "rayon")]
                {
                    value.sections.into_par_iter().map(Page::from).collect()
                }

                #[cfg(not(feature = "rayon"))]
                {
                    value.sections.into_iter().map(Page::from).collect()
                }
            },
        }
    }
}

/// Represents the deserialized target of the layout file (`Report/Layout`) within a Power BI report file (.pbix).
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawReport {
    /// The identifier of the visual theme associated with the report.
    theme: String,
    /// The pages of the report.
    sections: Vec<RawPage>,
}

/// Represents a page within a Power BI report.
pub struct Page {
    /// The unique identifier of the page.
    pub id: String,
    /// The public, display name of the page (visible in the report navigation menu).
    pub name: String,
    pub display_option: usize,
    /// Whether the page is hidden (`true`) or not (`false`).
    pub hidden: bool,
    /// The width of the page (in pixels).
    pub width: usize,
    /// The height of the page (in pixels).
    pub height: usize,
    /// The index of the page. This determins the order that pages appear in the report navigation menu.
    ///
    /// Pages with lower values appear before pages with higher values.
    pub ordinal: usize,
    /// The instances of visuals within the page.
    pub visuals: Vec<Visual>,
}

impl From<RawPage> for Page {
    fn from(value: RawPage) -> Self {
        Self {
            id: value.name,
            name: value.display_name,
            display_option: value.display_option,
            hidden: false,
            width: value.width,
            height: value.height,
            ordinal: value.ordinal,
            visuals: {
                #[cfg(feature = "rayon")]
                {
                    value
                        .visual_containers
                        .into_par_iter()
                        .map(Visual::from)
                        .collect()
                }

                #[cfg(not(feature = "rayon"))]
                {
                    value
                        .visual_containers
                        .into_iter()
                        .map(Visual::from)
                        .collect()
                }
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawPage {
    name: String,
    display_name: String,
    display_option: usize,
    width: usize,
    height: usize,
    ordinal: usize,
    visual_containers: Vec<RawVisual>,
}

pub struct Visual {
    pub x: usize,
    pub raw_x: f64,
    pub y: usize,
    pub raw_y: f64,
    pub width: usize,
    pub raw_width: f64,
    pub height: usize,
    pub raw_height: f64,
    pub layer: usize,
    pub tab_order: Option<usize>,
}

impl From<RawVisual> for Visual {
    fn from(value: RawVisual) -> Self {
        Self {
            x: value.x.round() as usize,
            raw_x: value.x,
            y: value.y.round() as usize,
            raw_y: value.y,
            width: value.width.round() as usize,
            raw_width: value.width,
            height: value.height.round() as usize,
            raw_height: value.height,
            layer: value.z,
            tab_order: value.tab_order,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawVisual {
    x: f64,
    y: f64,
    z: usize,
    width: f64,
    height: f64,
    tab_order: Option<usize>,
}

pub fn from_bytes(bytes: &[u8]) -> Result<Report, Box<dyn std::error::Error>> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))?;

    // Initialise a buffer for reading file data
    let mut buffer = Vec::new();

    // Extract and deserialise the report's layout metadata
    let raw_report: Option<RawReport> = extract(&mut archive, "Report/Layout", &mut buffer)
        .map_err(|e| format!("Failed to extract report layout data from .pbix: {e}"))?;

    // Check if the report layout data was present in the .pbix and successfully deserialised
    match raw_report {
        Some(raw_report) => Ok(raw_report.into()),
        None => Err("'Report/Layout' file not found in .pbix file".into()),
    }
}

/// Parses a Power BI report file (.pbix) and returns a `Report`.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Report, Box<dyn std::error::Error>> {
    // Read the .pbix file's bytes into a buffer
    let bytes = std::fs::read(path)
        .map_err(|e| IoError::new(e.kind(), format!("Failed to open .pbix file: {e}")))?;

    from_bytes(&bytes)
}

// Extracts and deserialises a JSON file (encoded in UTF-16LE) from a zip archive
pub fn extract<T>(
    archive: &mut ZipArchive<T>,
    file: &str,
    buffer: &mut Vec<u8>,
) -> Result<Option<RawReport>, Box<dyn std::error::Error>>
where
    T: std::io::Seek,
    T: std::io::Read,
{
    // Attempt to fetch file entry from zip archive
    let mut entry = match archive.by_name(file) {
        Ok(entry) => entry,
        Err(_) => return Ok(None),
    };

    // Extract raw file data from zip archive
    buffer.clear();
    buffer.reserve(entry.size() as usize); // Reserve capacity based on uncompressed size
    entry.read_to_end(buffer)?;

    // Decode the UTF-16LE encoded data
    let decoded: String = decode_utf16(
        buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])),
    )
    .filter_map(|r| match r {
        Ok(c) if !c.is_control() => Some(c), // Filter out control characters
        _ => None,
    })
    .collect();

    // Deserialize the JSON file
    let json: RawReport = serde_json::from_str(decoded.as_str())?;
    Ok(Some(json))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_file() {
        let report = super::parse_file("Example.pbix").unwrap();
        assert_eq!(report.pages.len(), 10);
    }

    #[test]
    fn from_bytes() {
        let pbix_data = include_bytes!("../Example.pbix");
        let report = super::from_bytes(pbix_data).unwrap();
        assert_eq!(report.pages.len(), 10);
    }
}
