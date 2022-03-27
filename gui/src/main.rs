pub mod auth;
pub mod bootstrap;
pub mod registry;
pub mod registry_repository;

extern crate magic_crypt;

pub mod config;
pub mod cryptor;
pub mod form;
pub mod settings;
pub mod support;
pub mod user;

pub mod resource;
pub mod resource_add;
pub mod resource_list;
pub mod menu;

use crate::{
    registry::registry::Registry, registry_repository::registry_repository::RegistryRepository,
};
use auth::form::form::AuthForm;
use form::{form::FormName, main_form::main_form::GUI};
use resource_list::form::resource_list_form::ResourceListForm;
use std::sync::Mutex;
use tracing_subscriber;

use eframe::{
    egui::{self, CentralPanel, Ui, Vec2, FontData},
    epi::App,
    run_native, NativeOptions,
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGISTRY: Mutex<RegistryRepository> = Mutex::new(RegistryRepository::new());
}

impl GUI {
    fn preset(ctx: &eframe::egui::Context, ui: &mut Ui) {
        ctx.set_visuals(egui::Visuals::dark());

        let spacing = ui.spacing_mut();
        spacing.button_padding = Vec2::new(10., 5.);
        spacing.indent_ends_with_horizontal_line =true;
        

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "JetBrainsMonoMedium".to_owned(),
        FontData::from_static(include_bytes!("../../JetBrainsMono-Medium.ttf"))
    );
        // fonts.family_and_size.insert(
        //     egui::TextStyle::Body,
        //     (egui::FontFamily::Proportional, 15.0),
        // );
        // fonts.family_and_size.insert(
        //     egui::TextStyle::Button,
        //     (egui::FontFamily::Proportional, 15.0),
        // );
        
        ctx.set_fonts(fonts);
    }

    fn render_header(ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            if Registry::eq_current_form(FormName::Auth) {
                ui.heading("Для начала нужно авторизоваться");
                ui.add_space(50.)
            }
        });
    }
}

impl App for GUI {
    fn setup(
        &mut self,
        _ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            GUI::preset(ctx, ui);

            GUI::render_header(ui);

            if Registry::eq_current_form(FormName::Auth) {
                GUI::render(AuthForm::new(), ui, ctx);
            }

            if Registry::eq_current_form(FormName::ResourceList) {
                ui.horizontal(|ui: &mut Ui| GUI::render(ResourceListForm::new(), ui, ctx));
            }
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
    win_options.initial_window_size = Some(Vec2::new(800., 580.));

    run_native(Box::new(app), win_options);
}


