use imgui::*;
use imgui_glow_renderer::{
    glow::{self, HasContext},
    AutoRenderer,
};
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("Example App", 1280, 720)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();

    let mut running = true;
    let mut tab_open = true;
    let mut event_pump = sdl.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);
            if let Event::Quit { .. } = event {
                running = false;
            }
        }

        platform.prepare_frame(&mut imgui, &window, &event_pump);
        let ui = imgui.new_frame();

        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(file_menu) = ui.begin_menu("File") {
                if ui.menu_item("Hello") {
                    println!("world!");
                }
                file_menu.end();
            }
            if let Some(help_menu) = ui.begin_menu("Help") {
                if ui.menu_item("Help") {
                    println!("no help :(");
                }
                help_menu.end();
            }
            menu_bar.end();
        }

        ui.window("Example App")
            .size(ui.io().display_size, Condition::Always)
            .position([0.0, 20.0], Condition::Always)
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .build(|| {
                ui.columns(2, "MainColumns", true);
                ui.text("Right");
                ui.next_column();

                if let Some(tab_bar) = ui.tab_bar("tab") {
                    if tab_open {
                        if let Some(_tab) = ui.tab_item_with_flags(
                            "one",
                            Some(&mut tab_open),
                            TabItemFlags::empty(),
                        ) {
                            let mut text_buffer = String::new();
                            ui.input_text_multiline(
                                "##",
                                &mut text_buffer,
                                ui.content_region_avail(),
                            )
                            .build();
                        }
                    }
                    if let Some(_tab) = ui.tab_item("two") {
                        ui.text_colored([255.0, 0.0, 0.0, 255.0], "Error?");
                    }
                    tab_bar.end();
                }
            });

        let draw_data = imgui.render();
        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();
        window.gl_swap_window();
    }
}
