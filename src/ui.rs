use slint::{SharedString, ModelRc, VecModel};
use std::{fs, rc::Rc, process::Command, ops::Add};
slint::include_modules!();

fn show_info(start_time: &str, end_time: &str, input_file: &str){
    println!("start_time: {}", start_time);
    println!("end_time: {}", end_time);
    println!("input_file: {}", input_file);
}

fn clip_video(start_time: &str, end_time: &str, input_file: &str){
    let output_file = input_file.replace(".mp4", "_clip.mp4");
    let command = format!("ffmpeg -ss {} -to {} -i {} {}", start_time, end_time, input_file, output_file);
    let output = Command::new("powershell")
        .arg(command)
        .output()
        .expect("Failed to execute process");

    println!("output: {:?}", String::from_utf8_lossy(&output.stdout));
    show_info(start_time, end_time, input_file);
}

fn find_mp4_files(directory: &str) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(directory)?;

    let mp4_files: Vec<String> = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                let extension = path.extension()?.to_string_lossy().to_lowercase();
                if extension == "mp4" {
                    Some(path.file_stem()?.to_string_lossy().to_string().add(".mp4"))
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(mp4_files)
}

pub fn ui() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    let ui = ui_handle.unwrap();

    let mp4_files = find_mp4_files(".").unwrap();

    let shared_mp4_files = mp4_files.into_iter().map(|s| SharedString::from(s)).collect::<Vec<SharedString>>();

    let the_model = ModelRc::new(Rc::new(VecModel::from(shared_mp4_files)).clone());

    ui.set_file_list(the_model);

    ui.on_clip_video(move || {
        let ui = ui_handle.unwrap();

        clip_video(&ui.get_start_time(), &ui.get_end_time(), &ui.get_input_file());
    });

    ui.run()
}