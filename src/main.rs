use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example")]
/// Write your journal
struct Opt {
    #[structopt()]
    /// path to the audio book to play
    book_path: Option<String>,
    #[structopt(name = "chapter", long, short)]
    /// chapter of the book to play
    chapter: Option<u32>,
    #[structopt(name = "length", long, short, default_value="1")]
    /// number of chapter to play
    length: u32,
    #[structopt(name = "back", short, long, default_value="0")]
    /// number of chapter to go back
    back: u32,
}

fn main() {
    let config_dir = format!("{}/.audiobooksrs", env::var("HOME").unwrap());
    fs::create_dir_all(config_dir).unwrap();

    let opt = Opt::from_args();
    let file_to_play = match opt.book_path {
        Some(p) => p,
        None => read_last_played()
    };
    let file_path = Path::new(file_to_play.as_str());
    let filename = file_path.file_stem().unwrap().to_str().unwrap();
    let progress = match opt.chapter {
        Some(x) => x,
        None => read_progress(filename),
    };
    play_audiobook(file_path, progress - opt.back, opt.length);
    write_progress(filename, progress + opt.length);
    write_last_played(file_path.to_str().unwrap());
}

fn play_audiobook(path: &Path, current_chapter: u32, number_of_chapter: u32) {
    let start_arg = &format!("--start=#{}", current_chapter);
    let end_arg = &format!("--end=#{}", current_chapter + number_of_chapter);
    Command::new("mpv")
        .arg("--no-audio-display")
        .arg(start_arg)
        .arg(end_arg)
        .arg(path)
        .status()
        .unwrap();
}

fn write_progress(name: &str, chapter: u32) {
    let file = format!("{}/.audiobooksrs/{}", env::var("HOME").unwrap(), name);
    fs::write(file, chapter.to_string()).unwrap();
}

fn read_progress(name: &str) -> u32 {
    let file = format!("{}/.audiobooksrs/{}", env::var("HOME").unwrap(), name);
    let progress = fs::read_to_string(file).unwrap_or_else(|_| {
        write_progress(name, 1);
        String::from("1")
    });
    progress.parse().unwrap()
}

fn read_last_played() -> String {
    let file = format!("{}/.audiobooksrs/last", env::var("HOME").unwrap());
    let last = fs::read_to_string(file).unwrap_or(String::new());
    last
}

fn write_last_played(name: &str) {
    let file = format!("{}/.audiobooksrs/last", env::var("HOME").unwrap());
    fs::write(file, name.to_string()).unwrap();
}
