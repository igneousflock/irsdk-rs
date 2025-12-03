use irsdk::IbtFile;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: dump_file <PATH_TO_IBT>");
    let file = IbtFile::from_file(&file_name).expect("could not open IBT file");

    println!("{:#?}", &file.header);
    println!("{:#?}", &file.disk_sub_header);
    println!("{:#?}", &file.var_buf_info);
}
