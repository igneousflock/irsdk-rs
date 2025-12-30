use ibt::IbtFile;

const USAGE: &str = "Usage: dump_var <PATH_TO_IBT> <VAR_NAME>";

fn main() {
    let file_name = std::env::args().nth(1).expect(USAGE);
    let var_name = std::env::args().nth(2).expect(USAGE);
    let file = IbtFile::from_file(&file_name).expect("could not open IBT file");

    let var = file.vars.var(&var_name).expect("unknown var");

    for sample in file.samples().take(500) {
        println!("{:?}", sample.read_var(var));
    }
}
