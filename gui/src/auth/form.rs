pub mod form {
    use std::{sync::{Mutex, MutexGuard}, borrow::Borrow};

    use eframe::egui::{
        self,
        Button,
        CtxRef,
        Label,
        Layout,
        TopBottomPanel,
        Ui,
        //  ScrollArea, Separator, TextBuffer,   Vec2, CentralPanel, Hyperlink, Color32,
    };

    use crate::{
        auth::{
            auth::{LastUser, Login},
            auth_service::auth_service::AuthService, auth_data::auth_data::AuthData,
        },
        form::form::{Form, FormName},
        registry::registry::Registry,
        REGISTRY, registry_repository::registry_repository::RegistryRepository,
    };
    pub const PADDING: f32 = 5.0;

    pub struct AuthForm {}

    impl AuthForm {
        pub fn new() -> AuthForm {
            AuthForm {}
        }

        pub fn render_check_user_panel(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(30.);
                tracing::error!("render_check_user_panel render");
                // let registry = &REGISTRY.lock().unwrap();
                // let auth_data = registry.auth_data;
                tracing::error!("after reg ");
                ui.horizontal(|ui: &mut Ui| {
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

                // login field
                ui.text_edit_singleline(&mut REGISTRY.lock().unwrap().auth_data.login);
                tracing::error!("after text_edit_singleline ");
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
            // let registry = &REGISTRY.lock().unwrap();
            //     let auth_data = registry.auth_data;

            let registry = &mut REGISTRY.lock().unwrap();
            let login = format!("▶ выбрать {}", registry.last_user.login);
            tracing::error!("last: {:?} ", login);
            let last_login_label = ui.button(login);
            if last_login_label.clicked() {
                registry.auth_data.login = registry.last_user.login.clone();
                tracing::error!("clicked {}", registry.auth_data.login);
            }
            tracing::error!("after render_users ");

            // {
            //     drop(registry);
            // };
        }

        pub fn render_buttons(&self, ui: &mut eframe::egui::Ui) {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(50.);
                tracing::error!("render_buttons");
                // let registry = &REGISTRY.lock().unwrap();
                // tracing::error!("render_buttons login");
                // let auth_data = &registry.auth_data;
                // tracing::error!("render_buttons 2");
                // let mut pass = auth_data.pass.lock().unwrap();
                // tracing::error!("render_buttons 3");
                // let mut login = auth_data.login;

                tracing::error!("render_buttons login");
                ui.text_edit_singleline(&mut *REGISTRY.lock().unwrap().auth_data.pass.lock().unwrap());
                if !ui.button("Ввод").clicked() {
                    tracing::error!("render_buttons 4");
                    ui.label(REGISTRY.lock().unwrap().auth_data.error_msg());
                    return;
                }

                tracing::error!("render_buttons 5");
                // let mut last_users_list = Vec::new();
                let mut user = LastUser::new();
                user.login = REGISTRY.lock().unwrap().auth_data.login.clone();
                // last_users_list.push(&user);
                tracing::error!("Saving error  {:?}", user);
                if let Err(e) = confy::store_path("./last-users-list.tmp", user) {
                    tracing::error!("Saving error  {}", e);
                }

                let auth_service = AuthService::new();
                let login = REGISTRY.lock().unwrap().auth_data.login.clone();
                let pass = REGISTRY.lock().unwrap().auth_data.pass.lock().unwrap().clone();

                tracing::error!("render_buttons 6");
                tracing::error!("enter_type_eq");
                if REGISTRY.lock().unwrap().auth_data.enter_type_eq(Login::Login) {
                    tracing::error!(
                        "попытка авторизации {:?}",
                        REGISTRY.lock().unwrap().auth_data.enter_type
                    );

                    
                    tracing::error!("login login ");
                    match auth_service.authenticate(
                        login,
                    pass,
                    ) {
                        Ok(r) => {
                            Registry::set_current_form(FormName::ResourceList);
                            r
                        }
                        Err(e) => {
                            tracing::error!("reg error {}", e);
                            REGISTRY.lock().unwrap().auth_data.set_error_msg(e.clone());
                            tracing::error!("reg error {}", e);
                            e.to_string()
                        }
                    };

                    return;
                }

                tracing::error!("попытка регистрации");
                ui.label("попытка регистрации");

                match auth_service.reg(
                    login,
                    pass,
                ) {
                    Ok(r) => {
                        Registry::set_current_form(FormName::ResourceList);
                        r
                    }
                    Err(e) => {
                        REGISTRY.lock().unwrap().auth_data.set_error_msg(e.clone());
                        tracing::error!("reg error {}", e);
                        e.to_string()
                    }
                };
            });
        }
    }

    impl Form for AuthForm {
        fn render(&self, ui: &mut eframe::egui::Ui) {
            ui.horizontal(|ui: &mut Ui| {
                ui.group(|ui| {
                    ui.set_max_width(300.0);
                    ui.set_max_height(300.0);
                    tracing::error!("AuthForm render");
                    self.render_check_user_panel(ui);

                    // ui.set_min_height(10.0);
                    // ui.set_max_width(20.0);
                });

                self.render_buttons(ui);
            });
        }
    }
}
