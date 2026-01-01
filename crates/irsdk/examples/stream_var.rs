#[cfg(target_family = "windows")]
mod windows {
    use std::time::Duration;

    use irsdk::IRacingClient;

    const USAGE: &str = "Usage: stream_var <VAR_NAME>";

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let var_name = std::env::args().nth(1).expect(USAGE);
        let client = IRacingClient::connect()?;

        let mut buf = vec![0; client.buf_len()];
        let var = client.vars().var(&var_name).expect("unknown var");

        loop {
            match client.next_sample_into_buf(&mut buf) {
                Ok(sample) => {
                    println!("{:?}", sample.read_var(var));
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(err) => {
                    println!("{err:#?}");
                    println!("{err}");
                    break;
                }
            }
        }

        Ok(())
    }
}

#[expect(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_family = "windows")]
    windows::main()?;
    Ok(())
}
