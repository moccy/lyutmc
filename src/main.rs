use log::info;
use lyutmc::run;

fn main() {
    env_logger::init();
    info!("Starting LyutMC");

    const WINDOW_SIZE: [u32; 2] = [800, 800];
    let window_title = "LyutMC";

    pollster::block_on(run(window_title, WINDOW_SIZE));
}
