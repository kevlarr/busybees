//! A tool to read an input directory,
//! process all images (resize if large and generate thumbnails),
//! create database records for each image,
//! and link them to posts by searching for `<img>` tags in their content.

use busybees::{imaging, store::images::Image};
use futures::FutureExt;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{self, PgPool};
use std::{
    env,
    fmt,
    fs,
    io::Result as IoResult,
    ops,
    path::{Path, PathBuf},
};

lazy_static! {
    pub static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

#[derive(Debug)]
struct Filename(String);

impl Filename {
    fn is_thumbnail(&self) -> bool {
        self.0.starts_with("thumb.")
    }
}

impl ops::Deref for Filename {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct ProcessedImages {
    ok: Vec<Image>,
    err: Vec<FileLink>,
}

struct ImportedImages {
    ok: Vec<Filename>,
    err: Vec<Filename>,
}

/// General purpose error representing failure to process a directory entry
/// for metadata or open as image.
struct FileLinkError(String);

impl fmt::Display for FileLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FileLinkError {
    fn from(msg: String) -> FileLinkError {
        FileLinkError(msg)
    }
}

/// Helper class to extract filename from owned path and open as image
struct FileLink {
    existing_path: Box<Path>,
    existing_filename: Filename,
    encoded_path: Box<Path>,
    encoded_filename: Filename,
}

impl FileLink {
    /// Accepts an owned Path. Returns `Some<Self>` if the path is able to
    /// be loaded as an image with an accessible filename. Returns `None`
    /// otherwise.
    fn new(path: Box<Path>) -> Result<Self, FileLinkError> {
        let filename = Filename(
            path.file_name()
                .ok_or_else(|| "Filename is not present".to_owned())?
                .to_os_string()
                .into_string()
                .map_err(|_| "Filename is not valid unicode".to_owned())?,
        );

        if filename.is_thumbnail() {
            return Err("File is a thumbnail".to_owned().into());
        }

        let encoded_filename = Filename(WHITESPACE.replace_all(&*filename, "+").to_string());
        let encoded_path = path.with_file_name(&*encoded_filename).into_boxed_path();

        Ok(FileLink {
            existing_path: path,
            existing_filename: filename,
            encoded_path,
            encoded_filename,
        })
    }
}

#[actix_rt::main]
async fn main() -> IoResult<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::new(&database_url)
        .now_or_never()
        .unwrap()
        .unwrap();

    let args: Vec<String> = std::env::args().collect();
    let dirpath = args
        .iter()
        .nth(1)
        .expect("Must provide images directory");

    if !Path::new(dirpath).is_dir() {
        panic!("Must specify a valid directory");
    }

    let file_paths = load_paths(&dirpath)?;

    println!("Searching {} for images", dirpath);
    println!("Found {} files", file_paths.len());

    let processed = process_images(file_paths);

    println!("Processed {} images", processed.ok.len());
    println!("Failed to process {} images:", processed.err.len());

    let imported = import_images(&pool, processed.ok).await;

    println!("Imported {} images", imported.ok.len());
    println!("Failed to import {} images:", imported.err.len());

    Ok(())
}

fn load_paths(dir: &str) -> IoResult<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .map(|entry| entry.ok())
        .filter_map(|opt_entry| opt_entry)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect())
}

fn process_images(paths: Vec<PathBuf>) -> ProcessedImages {
    let mut ok = Vec::new();
    let mut err = Vec::new();

    for image_path in paths {
        println!("\n{:?}", image_path);

        let link = match FileLink::new(image_path.into_boxed_path()) {
            Ok(link) => link,
            Err(e) => {
                println!("\t{}", e);
                continue;
            }
        };

        if link.existing_path != link.encoded_path {
            println!(
                "\tRenaming {} to {}",
                *link.existing_filename, *link.encoded_filename,
            );

            if let Err(e) = fs::rename(&link.existing_path, &link.encoded_path) {
                eprintln!("\tError: {}", e);
                continue;
            }
        }

        println!("\tProcessing");

        let image = match imaging::process(&link.encoded_path) {
            Ok(image) => image,
            Err(e) => {
                eprintln!("\tError: {}", e);
                err.push(link);
                continue;
            }
        };

        ok.push(image);
    }

    ProcessedImages { ok, err }
}

async fn import_images(pool: &PgPool, images: Vec<Image>) -> ImportedImages {
    let mut ok = Vec::new();
    let mut err = Vec::new();

    for image in images {
        let mut tx = pool.begin().await.unwrap();
        let filename = Filename(image.filename.clone());

        let result = sqlx::query!(
            "
            insert into image (filename, thumbnail_filename, width, height, kb)
                values ($1, $2, $3, $4, $5)
                on conflict do nothing
                returning id
            ",
            image.filename,
            image.thumbnail_filename,
            image.width,
            image.height,
            image.kb,
        )
        .fetch_one(&mut *tx)
        .await;

        let image_id = match result {
            Ok(row) => row.id,
            Err(e) => {
                eprintln!("{:?}: {}", filename, e.to_string());
                err.push(filename);
                continue;
            }
        };

        let result = sqlx::query!(
            "
            insert into post_image (image_id, post_id)
                select $1, post.id
                from post
                where content ~~ $2
            ",
            image_id,
            format!("%src=\"/uploads/{}\"%", image.filename)
        )
        .execute(&mut *tx)
        .await;

        match result {
            Ok(_) => match tx.commit().await {
                Ok(_) => {
                    ok.push(filename);
                }
                Err(e) => {
                    eprintln!("Image {}: {}", image_id, e.to_string());
                    err.push(filename);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Image {}: {}", image_id, e.to_string());
                err.push(filename);
                continue;
            }
        }
    }

    ImportedImages { ok, err }
}
