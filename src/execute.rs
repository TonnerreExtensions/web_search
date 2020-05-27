use std::process::Command;

pub fn execute(id: &str) {
    Command::new("/usr/bin/open")
        .arg(id)
        .spawn()
        .expect("Failed to spawn");
}
