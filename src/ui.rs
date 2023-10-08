use egui_macroquad::{*, egui::{Context, TopBottomPanel, RichText, Color32, menu}}; 
use crate::globals::*;

pub struct UI {
    pointer_over: bool,
}


impl UI {

    pub fn new() -> Self {
        Self {
            pointer_over: false,
        }
    }

    pub fn process(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx);
        });
    }


    pub fn draw(&self) {
        egui_macroquad::draw();
    }

    fn build_top_menu(&mut self, egui_ctx: &Context) {
        let mut signals = mod_signals();
        TopBottomPanel::top("top_panel").default_height(100.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            
            menu::bar(ui, |ui| {
                ui.label(RichText::new("QUANTUM").heading().strong().color(Color32::RED));
                ui.separator();
                
                menu::menu_button(ui, RichText::new("QUANTS").strong(), |ui| {
                    if ui.button(RichText::new("Single").strong().color(Color32::BLUE)).clicked() {
                        signals.add_single_quant = true;
                        init_global_signals(signals);
                    }
                    if ui.button(RichText::new("Some").strong().color(Color32::RED)).clicked() {
                        signals.add_some_quants = true;
                        init_global_signals(signals);
                    }
                    if ui.button(RichText::new("Hex").strong().color(Color32::RED)).clicked() {
                        signals.add_hex_quant = true;
                        init_global_signals(signals);
                    }
                    if ui.button(RichText::new("Solid").strong().color(Color32::GREEN)).clicked() {

                    }
                });

                menu::menu_button(ui, RichText::new("WORLD").strong(), |ui| {
                    if ui.button(RichText::new("Reset All").strong().color(Color32::RED)).clicked() {
                        signals.reset_all = true;
                        init_global_signals(signals);
                    }
                });
            })
        });
    }

}