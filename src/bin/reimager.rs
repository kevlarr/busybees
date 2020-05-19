use ::busybees::imaging;

use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let dirpath = args.iter().skip(1).next().expect("Must provide images directory");
    let path = Path::new(dirpath);

    if !path.is_dir() {
        panic!("Must specify a valid directory");
    }

    let images: Vec<PathBuf> = fs::read_dir(&path)?
        .map(|entry| entry.ok())
        .filter_map(|opt_entry| opt_entry)
        .map(|entry| entry.path())
        .filter(|path| !path.is_dir())

        // Opening the image just to collect valid image filepaths is not
        // fantastic, but this is not performance sensitive, so...
        .map(|path| (image::open(path.clone()).map(|_| path).ok()))
        .filter_map(|opt_path| opt_path)
        .collect();

    println!("Searching {} for images", dirpath);
    println!("Found {} images", images.len());

    for imgpath in images {
        println!("{:?}", imgpath);

        if let Some(p) = imgpath.to_str() {
            println!("{}", p);
            let image = imaging::process(&p).unwrap();
        }
    }

    Ok(())
}
