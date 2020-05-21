use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use futures::StreamExt;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

use crate::{imaging, State};
use crate::models::Image;

#[derive(Serialize)]
pub struct UploadedImages {
    srcpaths: Vec<String>,
}

/// Streams each included image, saving each to the application upload path with
/// timestamp, generating relevant image `src` paths, and linking them to the given
/// post.  Each image will be resized and thumbnailed as appropriate.
pub async fn upload(
    mut payload: Multipart,
    path: web::Path<(String,)>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let mut srcpaths = Vec::new();

    while let Some(item) = payload.next().await {
        let timestamp = Utc::now().timestamp();
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .map(|f| format!("{}.{}", timestamp, f))
            .ok_or_else(|| MultipartError::Incomplete)?;

        srcpaths.push(format!("uploads/{}", filename));

        let filepath = format!("{}/{}", state.upload_path, filename);
        let filepath = Path::new(&filepath);

        save_file(&mut field, filepath).await?;

        let image = imaging::process(&filepath)
            .map_err(|e| HttpResponse::BadRequest().body(e.to_string()))?;

        Image::create(&state.pool, &path.0, image).await
            .map_err(|e| HttpResponse::BadRequest().body(e))?;
    }

    Ok(HttpResponse::Ok().json(UploadedImages { srcpaths }))
}

async fn save_file(field: &mut Field, filepath: &Path) -> Result<(), Error> {
    let filepath = filepath.as_os_str().to_os_string();
    let mut f = web::block(|| std::fs::File::create(filepath)).await?;

    while let Some(chunk) = field.next().await {
        let data = chunk?;

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}
