use eframe::egui;
struct MyApp {
    name: String,
    age: u32,
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: String::from("Arthur"),
            age: 0,
        }
    }
    fn theme_switcher(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("Dark").clicked() {
                ctx.set_visuals(egui::Visuals::dark());
            }
            if ui.button("Light").clicked() {
                ctx.set_visuals(egui::Visuals::light());
            }
        });
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("my egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!(
                "Hello, {}! You are {} years old.",
                self.name, self.age
            ));
            self.theme_switcher(ui, ctx);
            ui.image(egui::include_image!("test01.png"));
        });
    }
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let temp = Box::new(MyApp::new(cc));
            Ok(temp)
        }),
    );
    Ok(())
}
