use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{web, Error, HttpResponse};
use futures::StreamExt;
use serde::Serialize;
use std::io::Write;


#[derive(Serialize)]
pub struct UploadedImages {
    filepaths: Vec<String>,
}


pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error>  {
    let mut filepaths = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filepath = format!("www/public/uploads/{}", filename);
        filepaths.push(format!("public/uploads/{}", filename));

        // TODO async-std..?
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;

            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().json(UploadedImages { filepaths }))
}
