use std::{process::Command, io::{stdin, stdout, Write}};
slint::include_modules!();

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap(); // flush it to the screen

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    
    return input.trim().to_string()
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

fn ui(start_time: &str, end_time: &str, input_file: &str) -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    let ui = ui_handle.unwrap();
    ui.set_start_time(start_time.into());
    ui.set_end_time(end_time.into());
    ui.set_input_file(input_file.into());

    ui.on_clip_video(move || {
        let ui_copy = ui_handle.unwrap();

        clip_video(&ui_copy.get_start_time(), 
                &ui_copy.get_end_time(), &ui_copy.get_input_file());
    });

    ui.run()
}

fn main() {
    let start_time = get_input("Enter start time (format : HH:mm:ss): ");
    let end_time = get_input("Enter end time (format : HH:mm:ss) : ");
    let input_file = get_input("Enter input file (include .mp4) : ");


    if cfg!(target_os = "windows") {
        println!("This is windows");
        ui(&start_time, &end_time, &input_file).unwrap();
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