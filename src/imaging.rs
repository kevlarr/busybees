use crate::models::Image;

use image::{GenericImageView, ImageError};
use image::imageops::FilterType;
use std::{fmt, fs, path::{Path, PathBuf}};

/// Image handling errors relating to file I/O or image processing.
#[derive(Debug)]
pub enum ImagingError {
    ImageOpenError(ImageError),
    ResizeError(ImageError),
    ThumbnailError(ImageError),
    PathError(String),
}

impl fmt::Display for ImagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImagingError::*;

        match self {
            ImageOpenError(e) => write!(f, "Error opening image file: {}", e),
            ResizeError(e) => write!(f, "Error resizing image: {}", e),
            ThumbnailError(e) => write!(f, "Error generating thumbnail: {}", e),
        }
    }
}

/// Opens `imgpath` and resizes the image so that the maximum dimension is 1200px, 
/// maintaining the aspect ratio and overwriting the existing file. Additionally,
/// if the image is over 400px in either dimension, it will generate a thumbnail
/// with maximum dimension of 400px and aspect ratio preserved.
pub fn process(filepath: &Path) -> Result<Image, ImagingError> {
    use ImagingError::*;

    let img = image::open(filepath).map_err(|e| ImageOpenError(e))?;
    let (width, height) = img.dimensions();

    if width > 1200 || height > 1200 {
        img.resize(1200, 1200, FilterType::CatmullRom)
            .save(filepath)
            .map_err(|e| ResizeError(e))?;
    }

    let thumbnail_filename = if width > 400 || height > 400 {
        let thumbpath = thumbnail_path(filepath)?;

        img.resize(400, 400, FilterType::CatmullRom)
            .save(&thumbpath)
            .map_err(|e| ThumbnailError(e))?;

        Some(path_filename(&thumbpath)?)
    } else {
        None
    };

    // Failing to obtain file size is fine, so just discard any error
    let kb = fs::metadata(filepath).ok().map(|meta| (meta.len() / 1024) as i32);

    Ok(Image {
        filename: path_filename(filepath)?,
        thumbnail_filename,
        width: width as i16,
        height: height as i16,
        kb,
    })
}

fn path_filename(path: &Path) -> Result<String, ImagingError> {
    path.file_name()
        .ok_or_else(|| ImagingError::PathError("Filename not present".to_owned()))?
        .to_os_string()
        .into_string()
        .map_err(|_os_str| ImagingError::PathError("Filename is not valid".into()))
}

/// Generates a thumbnail path string from the given filepath.
fn thumbnail_path(filepath: &Path) -> Result<PathBuf, ImagingError> {
    let thumbpath = PathBuf::new();

    if let Some(parent) = filepath.parent() {
        thumbpath.push(parent);
    }

    thumbpath.push(format!("thumb.{}", path_filename(filepath)?));
    Ok(thumbpath)
}
