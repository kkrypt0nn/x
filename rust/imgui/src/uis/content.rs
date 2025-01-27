use imgui::{Condition, Ui};

pub fn render(ui: &Ui, running: &mut bool) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(file_menu) = ui.begin_menu("File") {
            if ui.menu_item("Open") {
                println!("Open clicked");
            }
            if ui.menu_item("Save") {
                println!("Save clicked");
            }
            if ui.menu_item("Exit") {
                *running = false;
            }
            file_menu.end();
        }
        if let Some(help_menu) = ui.begin_menu("Help") {
            if ui.menu_item("About") {
                println!("About clicked");
            }
            help_menu.end();
        }
        menu_bar.end();
    }

    ui.window("File Tree")
        .size([300.0, ui.io().display_size[1]], Condition::Always)
        .position([0.0, 20.0], Condition::Always)
        .movable(false)
        .resizable(false)
        .title_bar(false)
        .build(|| {
            ui.text("File Tree");
            ui.separator();
            if let Some(project_folder) = ui.tree_node("Project") {
                ui.bullet_text("README.md");
                project_folder.end();
            }
        });

    ui.window("Main Content")
        .size(
            [
                ui.io().display_size[0] - 300.0,
                ui.io().display_size[1] - 20.0,
            ],
            Condition::Always,
        )
        .position([300.0, 20.0], Condition::Always)
        .title_bar(false)
        .movable(false)
        .resizable(false)
        .build(|| {
            ui.text("Main Content");
            ui.separator();
            ui.text_wrapped("Hello world!");
        });
}
