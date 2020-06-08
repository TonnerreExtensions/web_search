use std::process::{Command, Stdio};

pub fn execute(id: &str) {
    Command::new("/usr/bin/open")
        .arg(id)
        .spawn()
        .expect("Failed to spawn");
}

#[cfg(target_os = "macos")]
pub fn preview(id: &str) {
    let preview_file = format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
   <key>URL</key>
   <string>{}</string>
</dict>
</plist>"#,
        id
    );
    let preview_file_path = std::env::temp_dir().join("preview_file.webloc");
    std::fs::write(&preview_file_path, preview_file).expect("Failed to write webloc");
    Command::new("/usr/bin/qlmanage")
        .arg("-p")
        .arg(preview_file_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn");
}

#[cfg(not(target_os = "macos"))]
pub fn preview(id: &str) {
    execute(id);
}
