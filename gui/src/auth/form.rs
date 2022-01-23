pub mod form {

    use std::sync::Mutex;

    use eframe::egui::{
        self, Button,   CtxRef,  Label, Layout, TopBottomPanel, Ui,
        //  ScrollArea, Separator, TextBuffer,   Vec2, CentralPanel, Hyperlink, Color32,
    };

    use crate::{
        auth::{
            auth::{LastUser, Login},
            auth_service::auth_service::{AuthService},
        },

        form::form::{FormName, Form},
        registry::registry::Registry,
        REGISTRY,
    };

    pub const PADDING: f32 = 5.0;
    

    pub struct GUI {
        pub lfd: LoginFormData,
    }

impl GUI {
    pub fn render(form: impl Form, ui: &mut eframe::egui::Ui) {
        form.render(ui);
    }
}

    pub struct LoginFormData {
        pub enter_type: Mutex<Login>,
        pub login: Mutex<String>,
        pub pass: Mutex<String>,
        pub error_msg: Mutex<String>,
    }

    impl LoginFormData {
        pub fn new() -> Self {
            Self {
                enter_type: Mutex::new(Login::Login),
                login: Mutex::new("".to_string()),
                pass: Mutex::new("".to_string()),
                error_msg: Mutex::new("".to_string()),
            }
        }

        pub fn enter_type_eq(&self, enter_type: Login) -> bool {
            *self.enter_type.lock().unwrap() == enter_type
        }

        pub fn set_enter_type(&self, enter_type: Login) {
            *self.enter_type.lock().unwrap() = enter_type
        }

        pub fn error_msg(&self) -> String {
            self.error_msg.lock().unwrap().clone()
        }

        pub fn set_error_msg(&self, err: String) {
            *self.error_msg.lock().unwrap() = err;
        }
    }

    impl GUI {
        pub fn new() -> GUI {
            GUI {
                lfd: LoginFormData::new(),
            }
        }

        pub fn render_check_user_panel(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(30.);

                ui.horizontal(|ui: &mut Ui| {
                    ui.selectable_value(
                        &mut *self.lfd.enter_type.lock().unwrap(),
                        Login::Login,
                        "Вход",
                    );

                    ui.selectable_value(
                        &mut *self.lfd.enter_type.lock().unwrap(),
                        Login::Register,
                        "Регистрация",
                    );
                });

                // login field
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
            let last_login_label = ui.button(login);
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
                    ui.label(self.lfd.error_msg());
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

                let auth_service = AuthService::new();

                if self.lfd.enter_type_eq(Login::Login) {
                    tracing::error!(
                        "попытка авторизации {:?}",
                        *self.lfd.enter_type.lock().unwrap()
                    );

                    match auth_service.authenticate(
                        self.lfd.login.lock().unwrap().clone(),
                        self.lfd.pass.lock().unwrap().clone(),
                    ) {
                        Ok(r) => {
                            Registry::set_current_form(FormName::ResourceList);
                            r
                        }
                        Err(e) => {
                            self.lfd.set_error_msg(e.clone());
                            tracing::error!("reg error {}", e);
                            e.to_string()
                        }
                    };

                    return;
                }

                tracing::error!("попытка регистрации");
                ui.label("попытка регистрации");

                match auth_service.reg(
                    self.lfd.login.lock().unwrap().clone(),
                    self.lfd.pass.lock().unwrap().clone(),
                ) {
                    Ok(r) => {
                        Registry::set_current_form(FormName::ResourceList);
                        r
                    }
                    Err(e) => {
                        self.lfd.set_error_msg(e.clone());
                        tracing::error!("reg error {}", e);
                        e.to_string()
                    }
                };
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
                        let _close_btn =
                            ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                        let refresh_btn =
                            ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));

                        if refresh_btn.clicked() {
                            Registry::set_current_form(FormName::Auth);
                        }
                        
                        let _theme_btn =
                            ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                    });
                });
                ui.add_space(10.);
            });
        }
    }
}
