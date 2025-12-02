use irsdk::IbtFile;
use saphyr::YamlEmitter;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: dump_file <PATH_TO_IBT>");
    let file = IbtFile::from_file(&file_name).expect("could not open IBT file");

    let session_data = file.session_data().expect("could not parse session data");
    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);
    emitter.compact(true);
    emitter
        .dump(&saphyr::Yaml::from(&session_data))
        .expect("could not serialize session data");
    println!("{output}");
}
