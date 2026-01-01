use csv::Writer;
use irsdk::IRacingClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = IRacingClient::connect()?;

    let mut writer = Writer::from_writer(std::io::stdout());
    for var in client.vars().all_vars() {
        writer
            .serialize(var)
            .expect("could not serialize var header as csv");
    }

    Ok(())
}
