use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use futures::StreamExt;
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
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let timestamp = Utc::now().timestamp();
        let realpath = format!("{}/{}.{}", &state.upload_path, timestamp, filename);
        srcpaths.push(format!("uploads/{}.{}", timestamp, filename));

        // TODO async-std..?
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(realpath)).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;

            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().json(UploadedImages { filepaths: srcpaths }))
}
