pub mod form {
    
    use std::{
        sync::Mutex,
    };

    use eframe::{
        egui::{
            self, Button, CentralPanel, Color32, CtxRef, Hyperlink, Label, Layout, ScrollArea,
            Separator, TopBottomPanel, Ui, Vec2, TextBuffer,
        },
    };

    use crate::{ REGISTRY, auth::{auth::{Login, LastUser}, auth_service::auth_service::{AuthService, self}}, user::user::User, user_repositiry_fs::user_repositiry_fs::UserRepositoryFS, config::config::Config, cryptor::cryptor::Cryptor, form::form::FormName, registry::registry::Registry};


    pub const PADDING: f32 = 5.0;
    const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

    pub struct GUI {
        pub lfd: LoginFormData,
    }

    

    pub struct LoginFormData {
        pub enter_type: Mutex<Login>,
        pub login: Mutex<String>,
        pub pass: Mutex<String>,
    }

    impl LoginFormData {
        pub fn new() -> Self {
            Self {
                enter_type: Mutex::new(Login::Login),
                login: Mutex::new("".to_string()),
                pass: Mutex::new("".to_string()),
            }
        }
    }

    impl GUI {
        pub fn new() -> GUI {
            // let iter = (0..2).map(|a| User {
            //     login: format!("{}", a),
            // });

            GUI {
                lfd: LoginFormData::new(),
            }
        }

        pub fn render_check_user_panel(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(30.);

                ui.horizontal(|ui: &mut Ui| {
                    if ui
                        .selectable_value(
                            &mut *self.lfd.enter_type.lock().unwrap(),
                            Login::Login,
                            "Вход",
                        )
                        .clicked()
                    {
                        // *self.lfd.enter_type.lock().unwrap() = Login::Login
                        // *ENTER_PROCEDURE.lock().unwrap() = Login::Login;
                    }

                    if ui
                        .selectable_value(&mut *self.lfd.enter_type.lock().unwrap(), Login::Register, "Регистрация",)
                        .clicked()
                    {
                        // *ENTER_PROCEDURE.lock().unwrap() = Login::Register;
                    }
                });

                ui.text_edit_singleline(&mut *self.lfd.login.lock().unwrap());

                self.render_users(ui);
            });
        }

        fn render_users(&self, ui: &mut eframe::egui::Ui) {
            ui.set_row_height(10.0);
            ui.set_width(300.0);

            // for a in &self.users {
            //     ui.add_space(PADDING);
            //     // render title
            //     let login = format!("▶ ddddddddddd {}", a.login);
            //     ui.colored_label(WHITE, login);

            //     ui.add_space(PADDING);
            //     ui.add(Separator::default());
            // }


                ui.add_space(PADDING);
                // render title
                let login = format!("▶ выбрать {}", &REGISTRY.lock().unwrap().last_user.login);
                let last_login_label = ui.button( login);
                if last_login_label.clicked() {
                    *self.lfd.login.lock().unwrap() = REGISTRY.lock().unwrap().last_user.login.clone();
                    tracing::error!("clicked {}", *self.lfd.login.lock().unwrap());
                }
        }

        

        pub fn render_buttons(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(50.);

                ui.text_edit_singleline(&mut *self.lfd.pass.lock().unwrap());
                if !ui.button("Ввод").clicked() {
                    return;
                }

                // let mut last_users_list = Vec::new();
                let mut user = LastUser::new();
                user.login = self.lfd.login.lock().unwrap().clone();
                // last_users_list.push(&user);
                tracing::error!("Saving error  {:?}", user);
                if let Err(e) = confy::store_path("./last-users-list.tmp", user) {
                    tracing::error!("Saving error  {}", e);
                }

                

                if (Login::Register.eq( &*self.lfd.enter_type.lock().unwrap())) {
                    tracing::error!("попытка регистрации");
                    ui.label("попытка регистрации");

                    let auth_service = AuthService::new();
                    match auth_service.reg(
                        self.lfd.login.lock().unwrap().clone(), 
                        self.lfd.pass.lock().unwrap().clone()
                    ) {
                        Ok(r) => {
                            Registry::set_current_form(FormName::ResourceList);
                            r
                        },
                        Err(e) => {
                            tracing::error!("Saving error  {}", e);
                            e.to_string()
                        }
                    };

                } else {
                    tracing::error!("попытка авторизации {:?}", *self.lfd.enter_type.lock().unwrap());
                    ui.label("попытка авторизации");
                    Registry::set_current_form(FormName::ResourceList);
                }
            });
        }

        pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
            // define a TopBottomPanel widget
            TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(10.);
                egui::menu::bar(ui, |ui| {
                    // logo
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                    });
                    // controls
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                        let refresh_btn =
                            ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                        let theme_btn = ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                    });
                });
                ui.add_space(10.);
            });
        }
    }
}
