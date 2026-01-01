#[cfg(target_family = "windows")]
mod windows {
    use csv::Writer;
    use irsdk::IRacingClient;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = IRacingClient::connect()?;

        let mut writer = Writer::from_writer(std::io::stdout());
        for var in client.vars().all_vars() {
            writer
                .serialize(var)
                .expect("could not serialize var header as csv");
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
