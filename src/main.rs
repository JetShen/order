use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let source_dir = "./";
    let files = fs::read_dir(source_dir)?;

    for file in files {
        if let Ok(file) = file {
            let file_path = file.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    let ext_str = ext.to_string_lossy().to_string();
                    let target_dir = format!("{}/{}", source_dir, ext_str);
                    let target_path = format!("{}/{}", target_dir, file_path.file_name().unwrap_or(OsStr::new("unknown")).to_string_lossy());

                    if !Path::new(&target_dir).exists() {
                        fs::create_dir(&target_dir)?;
                    }

                    let target_file_path = Path::new(&source_dir).join(target_path);
                    fs::rename(file_path, target_file_path)?;
                }
            }
        }
    }

    Ok(())
}
