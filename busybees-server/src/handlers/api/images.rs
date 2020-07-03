use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::error::Error as ActixError;
use actix_web::web;
use crate::{ApiResult, State};
use busybees::{
    imaging,
    store,
    deps::{
        chrono::Utc,
        futures::StreamExt,
        regex::Regex,
    },
};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
pub struct UploadResponse {
    images: Vec<UploadedImage>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedImage {
    filename: String,
    thumbnail_filename: Option<String>,
    image_id: i32
}

/// Streams each included image, saving each to the application upload path with
/// timestamp, generating relevant image `src` paths, and linking them to the given
/// post.  Each image will be resized and thumbnailed as appropriate.
pub async fn upload(
    mut payload: Multipart,
    path: web::Path<(String,)>,
    state: web::Data<State>,
) -> ApiResult<web::Json<UploadResponse>> {
    let mut images = Vec::new();
    let rgx = Regex::new(r"\s+")?;

    while let Some(item) = payload.next().await {
        let timestamp = Utc::now().timestamp();
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .map(|f| rgx.replace_all(f, "+"))
            .map(|f| format!("{}.{}", timestamp, f))
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filepath = format!("{}/{}", state.upload_path, filename);
        let filepath = Path::new(&filepath);

        save_file(&mut field, filepath).await?;

        let image = imaging::process(&filepath)?;
        let image_id = store::images::create(&state.pool, &path.0, image.clone())
            .await?;

        images.push(UploadedImage {
            image_id,
            filename: image.filename,
            thumbnail_filename: image.thumbnail_filename,
        });
    }

    Ok(web::Json(UploadResponse { images }))
}

async fn save_file(field: &mut Field, filepath: &Path) -> Result<(), ActixError> {
    let filepath = filepath.as_os_str().to_os_string();
    let mut f = web::block(|| std::fs::File::create(filepath)).await?;

    while let Some(chunk) = field.next().await {
        let data = chunk?;

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}
