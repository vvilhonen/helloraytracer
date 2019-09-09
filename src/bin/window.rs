use helloraytracer::cli::install_logging;
use helloraytracer::Setup;
use minifb::{Key, Window, WindowOptions};
use std::sync::mpsc::sync_channel;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Ray traces a random scene to given file
struct Opt {
    /// Width of the image in pixels
    #[structopt(long = "width", default_value = "400")]
    width: usize,
    /// Height of the image in pixels
    #[structopt(long = "height", default_value = "200")]
    height: usize,
    /// Number of samples
    #[structopt(long = "samples", default_value = "10")]
    samples: usize,
    /// Max depth of tracing
    #[structopt(long = "max-depth", default_value = "20")]
    max_depth: usize,
    /// Render debug view instead,
    #[structopt(long = "debug-view")]
    debug_view: bool,
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

    let mut window = Window::new(
        "helloraytracer",
        opt.width,
        opt.height,
        WindowOptions::default(),
    )
    .unwrap();
    let mut buffer: Vec<u32> = vec![0; opt.width * opt.height];

    let (tx, rx) = sync_channel(1);
    thread::spawn(move || {
        while tx.send(helloraytracer::render(&setup)).is_ok() {
            setup.camera.rotate_around_y(5);
        }
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok(img) = rx.try_recv() {
            img.fill_u32_buf(&mut buffer);
            window.update_with_buffer(&buffer).unwrap();
        } else {
            window.update();
        }
    }
}
