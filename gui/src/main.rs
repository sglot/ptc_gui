pub mod auth;
pub mod bootstrap;
pub mod registry;
pub mod registry_repository;

extern crate magic_crypt;

pub mod config;
pub mod cryptor;
pub mod user;
pub mod support;
pub mod form;

pub mod resource;
pub mod resource_list;
pub mod resource_add;

use std::sync::Mutex;
use auth::form::form::{ AuthForm};
use form::{form::FormName, main_form::main_form::GUI};
use resource_list::form::resource_list_form::ResourceListForm;
use crate::{ registry::registry::Registry, registry_repository::registry_repository::RegistryRepository};
use tracing_subscriber;

use eframe::{
    egui::{CentralPanel, Ui, Vec2},
    epi::App,
    run_native, NativeOptions,
};


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGISTRY: Mutex<RegistryRepository> = Mutex::new(RegistryRepository::new());
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

            if Registry::eq_current_form(FormName::Auth) {

                GUI::render(AuthForm::new(), ui);
                
            }

            if Registry::eq_current_form(FormName::ResourceList) {
                ui.horizontal(|ui: &mut Ui| {
                    // ui.button(self.lfd.pass.lock().unwrap());
                    // ui.label(self.lfd.pass.lock().unwrap());
                    GUI::render(ResourceListForm::new(), ui)
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
    
        if Registry::eq_current_form(FormName::Auth) {
            ui.heading("Для начала нужно авторизоваться");
            ui.add_space(50.)
        }
        
    });
}
