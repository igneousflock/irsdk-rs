use irsdk::IRacingClient;
use std::error::Error;
use std::time::Duration;

const USAGE: &str = "Usage: stream_var <VAR_NAME>";

fn main() -> Result<(), Box<dyn Error>> {
    let var_name = std::env::args().nth(1).expect(USAGE);
    let client = IRacingClient::connect()?;

    let var = client.vars().var(&var_name).expect("unknown var");

    while let Ok(sample) = client.next_sample() {
        println!("{:?}", sample.read_var(var));
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
