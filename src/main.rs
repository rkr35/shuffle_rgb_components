use anyhow::{anyhow, Result};
use log::info;

fn main() -> Result<()> {
    init_logger()?;
    log::logger().flush();
    Ok(())
}

fn init_logger() -> Result<()> {
    use simplelog::{
        ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
        WriteLogger,
    };
    use std::env;
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

        let cmd_args: Vec<String> = env::args().collect();
        info!("Command line arguments: \"{}\"", cmd_args.join(" "));

        Ok(())
    } else {
        Err(anyhow!("Unable to determine local time offset."))
    }
}
