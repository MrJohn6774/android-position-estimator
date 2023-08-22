use log::trace;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

fn run(mut event_loop: EventLoop<()>) {
    log::info!("Running mainloop...");

    // todo!("init vulkan instance");

    // It's not recommended to use `run` on Android because it will call
    // `std::process::exit` when finished which will short-circuit any
    // Java lifecycle handling
    event_loop.run_return(move |event, event_loop, control_flow| {
        log::info!("Received Winit event: {event:?}");

        *control_flow = ControlFlow::Wait;
        match event {
            Event::Resumed => {
                // app.resume(event_loop);
            }
            Event::Suspended => {
                log::info!("Suspended, dropping render state...");
                // app.render_state = None;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_size),
                ..
            } => {
                // app.configure_surface_swapchain();
                // Winit: doesn't currently implicitly request a redraw
                // for a resize which may be required on some platforms...
                // app.queue_redraw();
            }
            Event::RedrawRequested(_) => {
                log::info!("Handling Redraw Request");

                // if let Some(ref surface_state) = app.surface_state {
                //     if let Some(ref rs) = app.render_state {
                //         let frame = surface_state
                //             .surface
                //             .get_current_texture()
                //             .expect("Failed to acquire next swap chain texture");
                //         let view = frame
                //             .texture
                //             .create_view(&wgpu::TextureViewDescriptor::default());
                //         let mut encoder =
                //             rs.device
                //                 .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                //                     label: None,
                //                 });
                //         {
                //             let mut rpass =
                //                 encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                //                     label: None,
                //                     color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                //                         view: &view,
                //                         resolve_target: None,
                //                         ops: wgpu::Operations {
                //                             load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                //                             store: true,
                //                         },
                //                     })],
                //                     depth_stencil_attachment: None,
                //                 });
                //             rpass.set_pipeline(&rs.render_pipeline);
                //             rpass.draw(0..3, 0..1);
                //         }

                //         rs.queue.submit(Some(encoder.finish()));
                //         frame.present();
                //         surface_state.window.request_redraw();
                //     }
                // }
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
