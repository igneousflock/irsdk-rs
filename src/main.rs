use irsdk::IbtFile;

fn main() {
    let file = IbtFile::from_file("test-files/nxt-laguna.ibt").unwrap();
    dbg!(&file.header);
    dbg!(&file.sub_header);
}
