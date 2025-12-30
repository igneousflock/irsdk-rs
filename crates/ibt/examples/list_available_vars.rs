use csv::Writer;
use ibt::IbtFile;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: dump_file <PATH_TO_IBT>");
    let file = IbtFile::from_file(&file_name).expect("could not open IBT file");

    let mut writer = Writer::from_writer(std::io::stdout());
    for var in file.vars.all_vars() {
        writer
            .serialize(var)
            .expect("could not serialize var header as csv");
    }
}
