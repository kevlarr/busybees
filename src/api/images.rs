use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use futures::StreamExt;
use image::imageops::FilterType;
use serde::Serialize;
use std::io::Write;

use crate::State;

#[derive(Serialize)]
pub struct UploadedImages {
    filepaths: Vec<String>,
}

pub async fn upload(mut payload: Multipart, state: web::Data<State>) -> Result<HttpResponse, Error> {
    let mut srcpaths = Vec::new();

    while let Some(item) = payload.next().await {
        let timestamp = Utc::now().timestamp();
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| MultipartError::Incomplete)?;

        srcpaths.push(format!("uploads/{}.{}", timestamp, filename));

        let filepath = format!("{}/{}.{}", state.upload_path, timestamp, filename);
        let thumbpath = format!("{}/thumb.{}.{}", state.upload_path, timestamp, filename);

        save_file(&mut field, filepath.clone()).await?;

        let img = image::open(&filepath)
            .map_err(|e| HttpResponse::BadRequest().body(e.to_string()))?;

        img.resize(1200, 1200, FilterType::CatmullRom).save(&filepath)
            .map_err(|e| HttpResponse::BadRequest().body(e.to_string()))?;

        img.thumbnail(400, 400).save(&thumbpath)
            .map_err(|e| HttpResponse::BadRequest().body(e.to_string()))?;
    }

    Ok(HttpResponse::Ok().json(UploadedImages { filepaths: srcpaths }))
}

async fn save_file(field: &mut Field, filepath: String) -> Result<(), Error> {
    let mut f = web::block(|| std::fs::File::create(filepath)).await?;

    while let Some(chunk) = field.next().await {
        let data = chunk?;

        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }

    Ok(())
}
