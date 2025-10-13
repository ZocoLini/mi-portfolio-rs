use image::GenericImageView;
use image::imageops::FilterType;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    resize_icons();
}

fn resize_icons() {
    let icons_dir = Path::new("static/img/icon");

    if !icons_dir.exists() {
        return;
    }

    for entry in WalkDir::new(icons_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ["png", "jpg", "jpeg"].contains(&ext.to_lowercase().as_str()) {
                match image::open(path) {
                    Ok(img) => {
                        let (w, h) = img.dimensions();

                        if w > 50 || h > 50 {
                            println!("Resizing large icon: {} ({}x{})", path.display(), w, h);

                            let resized = img.resize(50, 50, FilterType::Lanczos3);
                            resized.save(path).expect("Failed to save resized image");
                        }
                    }
                    Err(e) => eprintln!("Error opening image {}: {}", path.display(), e),
                }
            }
        }
    }

    println!("cargo:rerun-if-changed=static/img/icon");
}
