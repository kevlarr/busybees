use busybees::State;
use busybees::imaging;
use busybees::models::Image;

use std::fs;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Filename(String);

struct ProcessedImages {
    ok: Vec<Image>,
    err: Vec<PathBuf>,
}

struct ImportedImages {
    ok: Vec<Filename>,
    err: Vec<Filename>,
}

#[actix_rt::main]
async fn main() -> IoResult<()> {
    let args: Vec<String> = std::env::args().collect();

    let dirpath = args.iter().skip(1).next().expect("Must provide images directory");

    if !Path::new(dirpath).is_dir() {
        panic!("Must specify a valid directory");
    }

    let state = State::new(dirpath.clone());
    let image_paths = load_images(&state)?;

    println!("Searching {} for images", state.upload_path);
    println!("Found {} images", image_paths.len());

    let processed = process_images(image_paths);

    println!("Processed {} images", processed.ok.len());
    println!("Failed to process {} images:", processed.err.len());

    let imported = import_images(&state, processed.ok).await;

    println!("Imported {} images", imported.ok.len());
    println!("Failed to import {} images:", imported.err.len());

    Ok(())
}

async fn import_images(state: &State, images: Vec<Image>) -> ImportedImages {
    let mut ok = Vec::new();
    let mut err = Vec::new();

    for image in images {
        let mut tx = state.pool.begin().await.unwrap();
        let filename = Filename(image.filename.clone());

        let result = sqlx::query!("
            insert into image (filename, thumbnail_filename, width, height, kb)
                values ($1, $2, $3, $4, $5)
                on conflict do nothing
                returning id",
            image.filename,
            image.thumbnail_filename,
            image.width,
            image.height,
            image.kb,
        ).fetch_one(&mut *tx).await;

        let image_id = match result {
            Ok(row) => row.id,
            Err(e) => {
                eprintln!("{:?}: {}", filename, e.to_string());
                err.push(filename);
                continue;
            }
        };

        let result = sqlx::query!("
            insert into post_image (image_id, post_id)
                select $1, post.id
                from post
                -- where ...",
            image_id
        ).execute(&mut *tx).await;

        match result {
            Ok(_) => match tx.commit().await {
                Ok(_) => {
                    ok.push(filename);
                },
                Err(e) => {
                    eprintln!("Image {}: {}", image_id, e.to_string());
                    err.push(filename);
                    continue;
                },
            },
            Err(e) => {
                eprintln!("Image {}: {}", image_id, e.to_string());
                err.push(filename);
                continue;
            },
        }
    }

    ImportedImages { ok, err }
}

fn process_images(img_paths: Vec<PathBuf>) -> ProcessedImages {
    let mut ok = Vec::new();
    let mut err = Vec::new();

    for imgpath in img_paths {
        let image = match imaging::process(&imgpath.as_path()) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("{:?}: {}", imgpath, e.to_string());
                err.push(imgpath);
                continue;
            },
        };

        ok.push(image);
    }

    ProcessedImages { ok, err }
}

fn load_images(state: &State) -> IoResult<Vec<PathBuf>> {
    Ok(fs::read_dir(&state.upload_path)?
        .map(|entry| entry.ok())
        .filter_map(|opt_entry| opt_entry)
        .map(|entry| entry.path())
        .filter(|path| !path.is_dir())

        // Opening the image just to collect valid image filepaths is not
        // fantastic, but this is not performance sensitive, so...
        .map(|path| (image::open(path.clone()).map(|_| path).ok()))
        .filter_map(|opt_path| opt_path)
        .collect())
}
