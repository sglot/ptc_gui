pub mod form {
    use eframe::egui::{
        self,
        Context,
        TextEdit,
        Ui,
    };

    use crate::{
        auth::{
            auth::{LastUser, Login},
            auth_service::auth_service::AuthService,
        },
        form::form::{Form, FormName},
        registry::registry::Registry,
        REGISTRY,
    };
    pub const ROW_PADDING: f32 = 5.0;
    pub const TOP_PADDING: f32 = 30.0;

    pub struct AuthForm {}

    impl AuthForm {
        pub fn new() -> AuthForm {
            AuthForm {}
        }

        pub fn render_check_user_panel(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(TOP_PADDING);

                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(ROW_PADDING);
                    ui.selectable_value(
                        &mut REGISTRY.lock().unwrap().auth_data.enter_type,
                        Login::Login,
                        "Вход",
                    );

                    ui.selectable_value(
                        &mut REGISTRY.lock().unwrap().auth_data.enter_type,
                        Login::Register,
                        "Регистрация",
                    );
                });

                ui.add_space(ROW_PADDING);

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // ui.add_space(ROW_PADDING);
                    ui.label("Логин:");
                    ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().auth_data.login);
                });

                ui.add_space(ROW_PADDING);

                self.render_users(ui);
            });
        }

        fn render_users(&self, ui: &mut eframe::egui::Ui) {
            ui.set_width(300.0);

            let registry = &mut REGISTRY.lock().unwrap();
            let login = format!("▶ выбрать {}", registry.last_user.login);
            let last_login_label = ui.button(login);
            if last_login_label.clicked() {
                registry.auth_data.login = registry.last_user.login.clone();
            }

            ui.add_space(ROW_PADDING);
        }

        pub fn render_buttons(&mut self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(TOP_PADDING);
                ui.add_space(TOP_PADDING);
                ui.add_space(TOP_PADDING / 3.5);

                ui.label("Пароль:");
                let pass_field = ui.add(
                    TextEdit::singleline(
                        &mut *REGISTRY.lock().unwrap().auth_data.pass.lock().unwrap(),
                    )
                    .password(true),
                );

                // let p = ui.text_edit_singleline(
                //     &mut *REGISTRY.lock().unwrap().auth_data.pass.lock().unwrap(),
                // );

                ui.add_space(ROW_PADDING);

                if !(ui.button("Ввод").clicked()
                    || (pass_field.lost_focus() && ui.input().key_pressed(egui::Key::Enter)))
                {
                    ui.label(REGISTRY.lock().unwrap().auth_data.error_msg());
                    return;
                }

                ui.add_space(ROW_PADDING);

                let mut user = LastUser::new();
                user.login = REGISTRY.lock().unwrap().auth_data.login.clone();

                if let Err(e) = confy::store_path("./last-users-list.tmp", user) {
                    tracing::error!("Saving error  {}", e);
                }

                let mut auth_service = AuthService::new();
                let login = REGISTRY.lock().unwrap().auth_data.login.clone();
                let pass = REGISTRY
                    .lock()
                    .unwrap()
                    .auth_data
                    .pass
                    .lock()
                    .unwrap()
                    .clone();

                if REGISTRY
                    .lock()
                    .unwrap()
                    .auth_data
                    .enter_type_eq(Login::Login)
                {
                    match auth_service.authenticate(login, pass) {
                        Ok(r) => {
                            Registry::set_current_form(FormName::ResourceList);
                            REGISTRY
                                .lock()
                                .unwrap()
                                .auth_data
                                .is_auth = true;
                            r
                        }
                        Err(e) => {
                            REGISTRY.lock().unwrap().auth_data.set_error_msg(e.clone());
                            tracing::error!("reg error {}", e);
                            e.to_string()
                        }
                    };

                    return;
                }

                ui.label("попытка регистрации");

                match auth_service.reg(login, pass) {
                    Ok(r) => {
                        Registry::set_current_form(FormName::ResourceList);
                        r
                    }
                    Err(e) => {
                        REGISTRY.lock().unwrap().auth_data.set_error_msg(e.clone());
                        e.to_string()
                    }
                };
            });
        }
    }

    impl Form for AuthForm {
        fn render(&mut self, ui: &mut eframe::egui::Ui, _ctx: &Context) {
            ui.horizontal(|ui: &mut Ui| {
                ui.group(|ui| {
                    ui.set_max_width(300.0);
                    ui.set_max_height(300.0);

                    self.render_check_user_panel(ui);
                });

                self.render_buttons(ui);
            });
        }
    }
}
