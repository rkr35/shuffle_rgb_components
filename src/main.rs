#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use log::{error, info};

const IN_PICTURE: &str = "cat.png";
const OUT_PICTURE: &str = "cat_shuffled.png";
const LOG: &str = "run.log";

fn main() -> Result<()> {
    use std::time::Instant;

    let start = Instant::now();

    init_logger()?;

    let run_result = run();

    if let Err(e) = &run_result {
        error!("{e:?}");
    }

    info!("Program took {:?}.", Instant::now() - start);
    log::logger().flush();
    run_result
}

fn run() -> Result<()> {
    use image::{DynamicImage, Pixel};
    use rand::seq::SliceRandom;

    let image = image::open(IN_PICTURE).context(IN_PICTURE)?;
    info!("Loaded {}", IN_PICTURE);

    let mut image = if let DynamicImage::ImageRgb8(image) = image {
        image
    } else {
        return Err(anyhow!("We only support 8-bit RGB without alpha."));
    };

    let mut rng = rand::thread_rng();

    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.channels_mut().shuffle(&mut rng);
    }

    image.save(OUT_PICTURE).context(OUT_PICTURE)?;
    info!("Saved {}", OUT_PICTURE);

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

    let file = File::options().append(true).create(true).open(LOG)?;
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
