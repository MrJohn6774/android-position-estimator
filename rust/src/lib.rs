use log::trace;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
};

pub mod vulkan;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

fn run(mut event_loop: EventLoop<()>) {
    log::info!("Running mainloop...");

    let mut app = vulkan::VkApplication::new(&event_loop);

    // It's not recommended to use `run` on Android because it will call
    // `std::process::exit` when finished which will short-circuit any
    // Java lifecycle handling
    event_loop.run_return(move |event, event_loop, control_flow| {
        log::info!("Received Winit event: {event:?}");

        *control_flow = ControlFlow::Wait;
        match event {
            Event::Resumed => {
                // todo!("create window here")
            }
            Event::Suspended => {
                log::info!("Suspended, dropping render state...");
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                app.recreate_swapchain = false;
            }
            Event::RedrawEventsCleared => {
                log::info!("Handling Redraw Events Cleared");
                app.handle_redraw_events_cleared();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event: _, .. } => {
                log::info!("Window event {:#?}", event);
            }
            _ => {}
        }
    });
}

fn _main(event_loop: EventLoop<()>) {
    run(event_loop);
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    let event_loop = EventLoopBuilder::with_user_event()
        .with_android_app(app)
        .build();
    _main(event_loop);
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info) // Default Log Level
        .parse_default_env()
        .init();

    let event_loop = EventLoopBuilder::new().build();
    _main(event_loop);
}
