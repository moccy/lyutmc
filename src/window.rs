use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
    window::Window,
};

pub fn toggle_fullscreen(window: &Window) {
    if window.fullscreen().is_none() {
        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
    } else {
        window.set_fullscreen(None);
    }
}

pub fn get_window_sizes(
    event_loop: &EventLoop<()>,
    window_size: [u32; 2],
) -> (LogicalSize<u32>, PhysicalSize<u32>) {
    let monitor = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
        .expect("Failed to find a monitor.");
    let dpi = monitor.scale_factor();
    let logical: LogicalSize<u32> = window_size.into();
    let physical: PhysicalSize<u32> = logical.to_physical(dpi);

    (logical, physical)
}
