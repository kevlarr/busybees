use image::{GenericImageView, ImageError};
use image::imageops::FilterType;
use std::{fmt, fs, io};

#[derive(Debug)]
pub enum ImagingError {
    ImageOpenError(ImageError),
    ResizeError(ImageError),
    ThumbnailError(ImageError),
    ThumbnailCopyError(io::Error),
}

impl fmt::Display for ImagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImagingError::*;

        match self {
            ImageOpenError(e) => write!(f, "Error opening image file: {}", e),
            ResizeError(e) => write!(f, "Error resizing image: {}", e),
            ThumbnailError(e) => write!(f, "Error generating thumbnail: {}", e),
            ThumbnailCopyError(e) => write!(f, "Error copying file to thumbnail: {}", e),
        }
    }
}

/// Opens `imgpath` and resizes the image so that the maximum dimension is 1200px, 
/// maintaining the aspect ratio and overwriting the existing file. Additionally,
/// if the image is over 400px in either dimension, it will generate a thumbnail
/// with maximum dimension of 400px and aspect ratio preserved. If the image is
/// under 400px in both dimensions, than a copy is simply made at `thumbpath`.
pub fn process(imgpath: &str, thumbpath: &str) -> Result<(), ImagingError> {
    use ImagingError::*;

    let img = image::open(imgpath).map_err(|e| ImageOpenError(e))?;
    let (width, height) = img.dimensions();

    if width > 1200 || height > 1200 {
        img.resize(1200, 1200, FilterType::CatmullRom)
            .save(&imgpath)
            .map_err(|e| ResizeError(e))?;
    }

    if width > 400 || height > 400 {
        img.resize(400, 400, FilterType::CatmullRom)
            .save(&thumbpath)
            .map_err(|e| ThumbnailError(e))?;
    } else {
        // For now, the server assumes every image has a corresponding thumbnail
        // at the expected path, so just copy it. It's small.
        fs::copy(&imgpath, &thumbpath).map_err(|e| ThumbnailCopyError(e))?;
    }

    Ok(())
}

/// Generates the expected thumbnail path for the given image path name.
/// **Note:** This should definitely go away once the magic of the "first
/// image" for an article goes away.
pub fn thumbnail_path(imgpath: &str) -> String {
    let mut parts: Vec<&str> = imgpath.rsplitn(2, '/').collect();

    parts.reverse();
    parts.insert(1, "/thumb.");

    parts.join("")
}
