pub enum MenuAction {
    None,
    Connect { server: String, username: String },
    Quit,
}

const LABEL_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 200, 200);

pub struct MainMenu {
    server_address: String,
    username: String,
    show_connect: bool,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            server_address: "localhost:25565".into(),
            username: "Steve".into(),
            show_connect: false,
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context) -> MenuAction {
        let mut action = MenuAction::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(30, 30, 30)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);

                    ui.label(
                        egui::RichText::new("Ferrite")
                            .size(64.0)
                            .color(egui::Color32::WHITE)
                            .strong(),
                    );

                    ui.add_space(12.0);

                    ui.label(
                        egui::RichText::new("A Minecraft client written in Rust")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(180, 180, 180)),
                    );

                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new("Heavy early development - just getting started!")
                            .size(14.0)
                            .color(egui::Color32::from_rgb(255, 200, 100))
                            .italics(),
                    );

                    ui.add_space(40.0);

                    if self.show_connect {
                        self.draw_connect_form(ui, &mut action);
                    } else {
                        if ui
                            .add_sized(
                                [220.0, 40.0],
                                egui::Button::new(egui::RichText::new("Direct Connect").size(18.0)),
                            )
                            .clicked()
                        {
                            self.show_connect = true;
                        }

                        ui.add_space(12.0);

                        if ui
                            .add_sized(
                                [220.0, 40.0],
                                egui::Button::new(egui::RichText::new("Quit Game").size(18.0)),
                            )
                            .clicked()
                        {
                            action = MenuAction::Quit;
                        }
                    }
                });
            });

        action
    }

    fn draw_connect_form(&mut self, ui: &mut egui::Ui, action: &mut MenuAction) {
        ui.set_max_width(300.0);

        ui.label(
            egui::RichText::new("Username")
                .size(14.0)
                .color(LABEL_COLOR),
        );
        ui.add_sized(
            [300.0, 30.0],
            egui::TextEdit::singleline(&mut self.username),
        );

        ui.add_space(8.0);

        ui.label(
            egui::RichText::new("Server Address")
                .size(14.0)
                .color(LABEL_COLOR),
        );
        let response = ui.add_sized(
            [300.0, 30.0],
            egui::TextEdit::singleline(&mut self.server_address),
        );

        ui.add_space(16.0);

        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

        ui.horizontal(|ui| {
            ui.add_space(40.0);

            if ui
                .add_sized(
                    [100.0, 35.0],
                    egui::Button::new(egui::RichText::new("Back").size(16.0)),
                )
                .clicked()
            {
                self.show_connect = false;
            }

            ui.add_space(16.0);

            if ui
                .add_sized(
                    [140.0, 35.0],
                    egui::Button::new(egui::RichText::new("Connect").size(16.0)),
                )
                .clicked()
                || enter_pressed
            {
                *action = MenuAction::Connect {
                    server: self.server_address.clone(),
                    username: self.username.clone(),
                };
            }
        });
    }
}
