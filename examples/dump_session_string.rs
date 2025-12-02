
use irsdk::IbtFile;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: dump_file <PATH_TO_IBT>");
    let file = IbtFile::from_file(&file_name).unwrap();

    dbg!(file.session_data().unwrap());
}
