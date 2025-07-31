/// Повертає категорію файлу за розширенням
pub fn get_category(extension: &str) -> &'static str {
    match extension {
        // Зображення
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" => "Images",

        // Документи
        "pdf" | "doc" | "docx" | "txt" | "md" | "odt" | "rtf" => "Documents",

        // Архіви
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "Archives",

        // Аудіо
        "mp3" | "wav" | "flac" | "aac" | "ogg" => "Audio",

        // Відео
        "mp4" | "mkv" | "avi" | "mov" | "webm" => "Video",

        // Код
        "rs" | "py" | "js" | "ts" | "html" | "css" | "cpp" | "c" | "java" | "cs" | "json" | "toml" | "yaml" => "Code",

        // Інші
        _ => "Other",
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_category() {
        assert_eq!(get_category("jpg"), "Images");
        assert_eq!(get_category("pdf"), "Documents");
        assert_eq!(get_category("zip"), "Archives");
        assert_eq!(get_category("mp3"), "Audio");
        assert_eq!(get_category("exe"), "Other");
    }
}


