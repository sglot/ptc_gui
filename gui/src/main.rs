pub mod auth;
pub mod bootstrap;
pub mod registry;

extern crate magic_crypt;

pub mod config;
pub mod cryptor;
pub mod user_repositiry_fs;
pub mod user;
pub mod support;
pub mod form;


use std::sync::Mutex;
use auth::form::form::GUI;
use form::form::FormName;
use crate::{auth::auth::Login, registry::registry::Registry};
use tracing::info;
use tracing_subscriber;

use eframe::{
    egui::{CentralPanel, ScrollArea, Ui, Vec2},
    epi::App,
    run_native, NativeOptions,
};


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGISTRY: Mutex<Registry> = Mutex::new(Registry::new());
}


impl App for GUI {
    fn setup(
        &mut self,
        _ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            render_header(ui);

            if (Registry::eq_current_form(FormName::Auth)) {
                ui.horizontal(|ui: &mut Ui| {
                    ui.group(|ui| {
                        ui.set_max_width(300.0);
                        ui.set_max_height(300.0);

                        self.render_check_user_panel(ui);

                        // ui.set_min_height(10.0);
                        // ui.set_max_width(20.0);
                    });

                    self.render_buttons(ui);
                });
            }

            if (Registry::eq_current_form(FormName::ResourceList)) {
                ui.horizontal(|ui: &mut Ui| {
                    ui.button(self.lfd.pass.lock().unwrap());
                    ui.label(self.lfd.pass.lock().unwrap());
                });
            }

            // ui.allocate_space(ui.available_size());
        });
    }

    fn name(&self) -> &str {
        "Password template cryptor"
    }
}
fn main() {
    tracing_subscriber::fmt::init();

    let app = GUI::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(640., 480.));

    run_native(Box::new(app), win_options);
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        // todo 
        // if check_auth => show login
        ui.heading("Для начала нужно авторизоваться");
        ui.add_space(50.)
    });
}
