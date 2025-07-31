use walkdir::WalkDir;
use std::fs;
use std::path::Path;

use crate::categories::get_category;
use anyhow::{Result, Context};

pub fn organize(dir: &str, dry_run: bool) -> Result<()> {
    let base_path = Path::new(dir);

    if !base_path.is_dir() {
        return Err(anyhow::anyhow!("Шлях {:?} не є директорією", dir));
    }

    for entry in WalkDir::new(base_path).min_depth(1).max_depth(1) {
        let entry = entry.with_context(|| "Помилка під час обходу директорії")?;
        let path = entry.path();

        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let category = get_category(&extension);
            let category_path = base_path.join(category);
            let target_path = category_path.join(
                path.file_name().unwrap_or_default()
            );

            if dry_run {
                println!("[Dry Run] Would move: {:?} -> {:?}", path, target_path);
            } else {
                fs::create_dir_all(&category_path)
                    .with_context(|| format!("Не вдалося створити директорію {:?}", category_path))?;

                fs::rename(&path, &target_path)
                    .with_context(|| format!("Не вдалося перемістити {:?} -> {:?}", path, target_path))?;
                
                println!("Moved: {:?} -> {:?}", path, target_path);
            }
        }
    }

    Ok(())
}

