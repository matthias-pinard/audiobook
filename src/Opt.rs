use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
/// Write your journal
struct Opt {
    #[structopt()]
    /// path to the audio book to play
    book_path: String,
    #[structopt(short = "c", long = "chapter")]
    /// chapter of the book to play
    chapter: Option<u32>,
    #[structopt(short = "l", long = "length", default_value="1")]
    /// number of chapter to play
    length: u32,
}
