use anyhow::{anyhow, Result};
use log::info;

fn main() -> Result<()> {
    init_logger()?;
    info!("test");
    log::logger().flush();
    Ok(())
}

fn init_logger() -> Result<()> {
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
        WriteLogger,
    };
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::options().append(true).create(true).open("run.log")?;
    let file = BufWriter::new(file);

    let level = LevelFilter::Debug;

    if let Ok(config) = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
    {
        CombinedLogger::init(vec![
            TermLogger::new(
                level,
                config.build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(level, config.build(), file),
        ])?;

        Ok(())
    } else {
        Err(anyhow!("Unable to determine local time offset."))
    }
}
