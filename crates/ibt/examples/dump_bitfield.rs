use ibt::{
    IbtFile,
    telemetry::{Value, VarType},
};

const USAGE: &str = "Usage: dump_bitfield <PATH_TO_IBT> <VAR_NAME>";

fn main() {
    let file_name = std::env::args().nth(1).expect(USAGE);
    let var_name = std::env::args().nth(2).expect(USAGE);
    let file = IbtFile::from_file(&file_name).expect("could not open IBT file");

    let var = file.vars.var(&var_name).expect("unknown var");
    assert_eq!(var.ty, VarType::Bitfield);

    for sample in file.samples() {
        let Value::Bitfield(bf) = sample.read_var(var) else {
            panic!()
        };
        println!("{bf:?}");
    }
}
