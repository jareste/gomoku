use egui::{CtxRef, CentralPanel};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::event_loop::{EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let platform = Platform::new(PlatformDescriptor {
        physical_width: window.inner_size().width,
        physical_height: window.inner_size().height,
        scale_factor: window.scale_factor(),
        font_definitions: Default::default(),
        style: Default::default(),
    });

    event_loop.run(move |event, _, control_flow| {
        // platform.handle_event(&event);
        match event {
            winit::event::Event::RedrawRequested(_) => {
                let mut egui_ctx = CtxRef::default();
                egui_ctx.begin_frame(egui::RawInput::default());

                egui::TopBottomPanel::top("top_panel").show(&egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Gomoku Game");
                    });
                });

                CentralPanel::default().show(&egui_ctx, |ui| {
                    ui.vertical(|ui| {
                        if ui.button("Button 1").clicked() {
                            // Do nothing
                        }
                        if ui.button("Button 2").clicked() {
                            // Do nothing
                        }
                        if ui.button("Button 3").clicked() {
                            // Do nothing
                        }
                    });
                });

                let (output, paint_commands) = egui_ctx.end_frame();
                // platform.end_frame(Some(&window));                // Handle output such as cursor changes
            }
            _ => (),
        }
    });
}