pub mod settings {
    use eframe::egui::Color32;

    pub const COLOR_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    pub const COLOR_RED: Color32 = Color32::from_rgb(250, 45, 45);
    pub const COLOR_GREEN: Color32 = Color32::from_rgb(45, 250, 127);
    pub const COLOR_BLUE: Color32 = Color32::from_rgb(78, 146, 250);
    pub const COLOR_BLACK: Color32 = Color32::from_rgb(0, 0, 0);
    pub const BG_COLOR_BUTTON: Color32 = Color32::from_rgb(60, 60, 60);


    pub const LIST_ROW_PADDING_BOTTOM: f32 = 10.;
    pub const EDIT_FIELD_PADDING_BOTTOM: f32 = 10.;
    pub const COLUMN_LEVEL_ONE_MARGIN: f32 = 30.;
    pub const COLUMN_LEVEL_TWO_MARGIN: f32 = 15.;
}
