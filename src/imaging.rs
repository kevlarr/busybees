use crate::models::Image;

use image::{GenericImageView, ImageError};
use image::imageops::FilterType;
use std::{fmt, fs};

/// Image handling errors relating to file I/O or image processing.
#[derive(Debug)]
pub enum ImagingError {
    ImageOpenError(ImageError),
    ResizeError(ImageError),
    ThumbnailError(ImageError),
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
pub fn process(imgpath: &str) -> Result<Image, ImagingError> {
    use ImagingError::*;

    let img = image::open(imgpath).map_err(|e| ImageOpenError(e))?;
    let (width, height) = img.dimensions();

    if width > 1200 || height > 1200 {
        img.resize(1200, 1200, FilterType::CatmullRom)
            .save(&imgpath)
            .map_err(|e| ResizeError(e))?;
    }

    let thumbnail_src = if width > 400 || height > 400 {
        let thumbpath = thumbnail_path(imgpath);

        img.resize(400, 400, FilterType::CatmullRom)
            .save(&thumbpath)
            .map_err(|e| ThumbnailError(e))?;

        Some(thumbpath)
    } else {
        None
    };

    // Failing to obtain file size is fine, so just discard any error
    let kb = fs::metadata(imgpath).ok().map(|meta| (meta.len() / 1024) as i32);

    Ok(Image {
        src: String::from(imgpath),
        thumbnail_src,
        width: Some(width as i16),
        height: Some(height as i16),
        kb,
    })
}

/// Returns a new path name with the filename portion of `imgpath`
/// prepended by `"thumb.'`.
pub fn thumbnail_path(imgpath: &str) -> String {
    let mut parts: Vec<&str> = imgpath.rsplitn(2, '/').collect();

    parts.reverse();
    parts.insert(1, "/thumb.");
    parts.join("")
}
