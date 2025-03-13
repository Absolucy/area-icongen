use crate::icongen::IconGenerator;
use eframe::{App, CreationContext, Frame, Storage};
use egui::{Context, TextEdit};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppDoohickey {
	text: Vec<String>,
	icongen: IconGenerator,
}

impl Default for AppDoohickey {
	fn default() -> Self {
		Self {
			text: vec![
				"123".to_owned(),
				"456".to_owned(),
				"789".to_owned(),
				"0".to_owned(),
			],
			icongen: IconGenerator::new([255, 115, 51, 120], [255, 115, 51, 255], [0, 0, 0, 255]),
		}
	}
}

impl AppDoohickey {
	/// Called once before the first frame.
	pub fn new(cc: &CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		let mut app = match cc.storage {
			Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
			None => Self::default(),
		};
		app.update_texture(&cc.egui_ctx);
		app
	}

	fn update_texture(&mut self, ctx: &Context) {
		self.icongen.update_texture(ctx, &self.text);
	}
}

impl App for AppDoohickey {
	fn save(&mut self, storage: &mut dyn Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.group(|ui| {
				let mut text_changed = false;
				if self.text.len() < 5 && ui.button("+").clicked() {
					self.text.push(String::new());
				}
				let mut to_remove = None;
				egui::Grid::new("text").num_columns(2).show(ui, |ui| {
					for (idx, line) in self.text.iter_mut().enumerate() {
						ui.horizontal(|ui| {
							if ui.add(TextEdit::singleline(line).char_limit(8)).changed() {
								text_changed = true;
							}
							if idx > 0 && ui.button("-").clicked() {
								to_remove = Some(idx);
								text_changed = true;
							}
						});
						ui.end_row();
					}
				});

				if let Some(idx) = to_remove {
					self.text.remove(idx);
				}

				if text_changed {
					self.update_texture(ctx);
				}
			});
			ui.group(|ui| {
				egui::Grid::new("colors").num_columns(2).show(ui, |ui| {
					ui.label("Background");
					ui.color_edit_button_srgba_unmultiplied(&mut self.icongen.background_color);
					ui.end_row();

					ui.label("Text Background");
					ui.color_edit_button_srgba_unmultiplied(
						&mut self.icongen.text_background_color,
					);
					ui.end_row();

					ui.label("Text");
					ui.color_edit_button_srgba_unmultiplied(&mut self.icongen.text_color);
					ui.end_row();
				});
			});

			//ui.group(|ui| ui.color_edit_button_srgba_premultiplied(srgba));

			self.icongen.show(ui);
		});
	}
}
