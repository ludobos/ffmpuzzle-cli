pub fn copy_to_clipboard(text: &str) -> bool {
    match arboard::Clipboard::new() {
        Ok(mut clip) => clip.set_text(text).is_ok(),
        Err(_) => false,
    }
}
