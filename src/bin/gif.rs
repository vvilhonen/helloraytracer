use gif::{Encoder, Frame, Repeat, SetParameter};
use helloraytracer::cli::install_logging;
use helloraytracer::Setup;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Ray traces a random scene to given GIF file
struct Opt {
    /// Width of the image in pixels
    #[structopt(long = "width", default_value = "640")]
    width: usize,
    /// Height of the image in pixels
    #[structopt(long = "height", default_value = "480")]
    height: usize,
    /// Number of samples
    #[structopt(long = "samples", default_value = "200")]
    samples: usize,
    /// Max depth of tracing
    #[structopt(long = "max-depth", default_value = "50")]
    max_depth: usize,
    /// Render debug view instead,
    #[structopt(long = "debug-view")]
    debug_view: bool,
    /// Path of the output file
    path: PathBuf,
}

fn main() {
    install_logging();

    let opt = Opt::from_args();
    let mut setup = Setup::from_render_opts(
        opt.debug_view,
        opt.width,
        opt.height,
        opt.samples,
        opt.max_depth,
    );
    let start = Instant::now();
    let fps = 60;
    let frames = (1..=fps)
        .map(|idx| {
            let frame = helloraytracer::render(&setup);
            log::info!("Rendered frame {}/{}", idx, fps);
            setup.camera.rotate_around_y(360 / fps);
            frame
        })
        .collect::<Vec<_>>();

    let gif = File::create(&opt.path).unwrap();
    let mut encoder = Encoder::new(gif, opt.width as u16, opt.height as u16, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    let mut upside_down = vec![0u8; opt.width * opt.height * 3];
    for frame in frames {
        for y in 0..opt.height {
            for x in 0..opt.width {
                let src = ((opt.height - y - 1) * opt.width + x) * 3;
                let dst = (y * opt.width + x) * 3;
                upside_down[dst + 0] = frame.data[src + 0];
                upside_down[dst + 1] = frame.data[src + 1];
                upside_down[dst + 2] = frame.data[src + 2];
            }
        }
        let mut gif_frame = Frame::from_rgb(frame.nx as u16, frame.ny as u16, &upside_down);
        gif_frame.delay = 4;
        encoder.write_frame(&gif_frame).unwrap();
    }

    let render_time = start.elapsed();

    log::info!(
        "Rendering took {} ms, wrote to {}",
        render_time.as_millis() as u32,
        opt.path.display()
    );
}
