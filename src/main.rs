use std::process::Command;

mod ui;
use ui::ui;

fn is_ffmpeg_installed() -> bool {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .ok();

    if let Some(output) = output {
        output.status.success()
    } else {
        false
    }
}

fn main() {
    if cfg!(target_os = "windows") {
        println!("This is windows");
        ui().unwrap();

        if is_ffmpeg_installed() {
        } else {
            println!("Error: ffmpeg is not installed. Please install ffmpeg and try again.");
        }
    } else if cfg!(target_os = "linux") {
        println!("This is linux");
        // todo
    } else if cfg!(target_os = "macos") {
        println
        !("This is macos");
        // todo
    } else {
        println!("This is unknown");
        // todo
    }
}