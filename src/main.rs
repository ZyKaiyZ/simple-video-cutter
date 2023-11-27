use std::process::Command;
slint::include_modules!();

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

fn clip_video(start_time: &str, end_time: &str, input_file: &str){
    let output_file = input_file.replace(".mp4", "_clip.mp4");
    let command = format!("ffmpeg -ss {} -to {} -i {} {}", start_time, end_time, input_file, output_file);
    let output = Command::new("powershell")
        .arg(command)
        .output()
        .expect("Failed to execute process");

    println!("output: {:?}", String::from_utf8_lossy(&output.stdout));
}

fn ui() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    let ui = ui_handle.unwrap();

    ui.on_clip_video(move || {
        let ui = ui_handle.unwrap();

        clip_video(&ui.get_start_time(), &ui.get_end_time(), &ui.get_input_file());
    });

    ui.run()
}

fn main() {
    if cfg!(target_os = "windows") {
        println!("This is windows");

        if is_ffmpeg_installed() {
            ui().unwrap();
        } else {
            println!("Error: ffmpeg is not installed. Please install ffmpeg and try again.");
        }
    } else if cfg!(target_os = "linux") {
        println!("This is linux");
        // todo
    } else if cfg!(target_os = "macos") {
        println!("This is macos");
        // todo
    } else {
        println!("This is unknown");
        // todo
    }
}