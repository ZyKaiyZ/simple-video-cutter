use std::{process::Command, io::Write};
use std::io::{stdin, stdout};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

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
        .expect("failed to execute process");

    println!("output: {:?}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let start_time = get_user_input("Enter start time: ");
    let end_time = get_user_input("Enter end time: ");
    let input_file = get_user_input("Enter input file: ");

    if cfg!(target_os = "windows") {
        println!("This is windows");
        clip_video(&start_time, &end_time, &input_file);
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
