mod calculator;

use eframe::egui::{self, CentralPanel, Context, Ui, Vec2, ViewportBuilder};
use calculator::{calculate};


fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/Montserrat-Medium.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}

struct CalculatorApp {
    expression: String,
}

impl CalculatorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            expression: String::new(),
        }
    }

    fn calculate(&mut self) {
        match calculate(&self.expression) {
            Ok(result) => {
                self.expression = result.to_string();
            },
            Err(e) => {
                self.expression = e.to_string();
            }
        }

    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Калькулятор");

            ui.add_sized(Vec2::new(285.0, 30.0), egui::TextEdit::singleline(&mut self.expression)
                .font(egui::FontId::monospace(20.0)));

            ui.vertical(|ui| {
                self.add_digit_buttons(ui);
            });
        });
    }

    fn add_digit_buttons(&mut self, ui: &mut Ui) {
        let button_size = Vec2::new(65.0, 65.0);
        let button_font_id = egui::FontId::monospace(22.0); // Set the desired font size

        let buttons_array = [
            ["pfx ", " ", "<-", "="],
            ["1", "2", "3", " + "],
            ["4", "5", "6", " - "],
            ["7", "8", "9", " * "],
            ["C", "0", ".", " / "]];

        for row in buttons_array.iter() {
            ui.horizontal(|ui| {
                for button in row.iter() {
                    if ui.add_enabled(true,
                                      egui::Button::new(
                                          egui::RichText::new(*button).font(button_font_id.clone()))
                                          .min_size(button_size))
                        .clicked() {
                            match *button {
                                "<-" => {
                                    self.expression.pop();
                                },
                                "=" => {
                                    self.calculate();
                                },
                                "C" => {
                                    self.expression.clear();
                                }
                                _ => {
                                    self.expression.push_str(button);
                                }
                            }
                    }
                }
            });
        }

    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}


fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([300.0, 400.0]).with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Калькулятор",
        options,
        Box::new(|_cc| {
            Ok(Box::new(CalculatorApp::new(_cc)))
        }),
    )
}
