use helloraytracer::cli::install_logging;
use helloraytracer::Setup;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Ray traces a random scene to given file
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
    let setup = Setup::from_render_opts(
        opt.debug_view,
        opt.width,
        opt.height,
        opt.samples,
        opt.max_depth,
    );
    let start = Instant::now();
    let data = helloraytracer::render(&setup);
    let render_time = start.elapsed();
    data.write_to_png(&opt.path);
    log::info!(
        "Rendering took {} ms, wrote to {}",
        render_time.as_millis() as u32,
        opt.path.display()
    );
}
