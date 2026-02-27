use crate::icongen::IconGenerator;
use eframe::{App, CreationContext, Frame, Storage};
use egui::{Context, TextEdit};
#[cfg(not(target_arch = "wasm32"))]
use egui_file_dialog::FileDialog;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppDoohickey {
	text: Vec<String>,
	icongen: IconGenerator,
	#[cfg(not(target_arch = "wasm32"))]
	#[serde(skip)]
	file_dialog: FileDialog,
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
			#[cfg(not(target_arch = "wasm32"))]
			file_dialog: FileDialog::new()
				.add_save_extension("PNG Image", "png")
				.default_save_extension("PNG Image"),
		}
	}
}

impl AppDoohickey {
	/// Called once before the first frame.
	pub fn new(cc: &CreationContext<'_>) -> Self {
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

	#[cfg(not(target_arch = "wasm32"))]
	fn save_png(&mut self) {
		self.file_dialog.config_mut().default_file_name = self.safe_filename();
		self.file_dialog.save_file();
	}

	#[cfg(target_arch = "wasm32")]
	fn save_png(&mut self) {
		use wasm_bindgen::prelude::*;
		use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

		let png_data = {
			let mut bytes = Vec::new();
			self.icongen
				.image()
				.write_to(
					&mut std::io::Cursor::new(&mut bytes),
					image::ImageFormat::Png,
				)
				.expect("failed to write png data");
			bytes
		};

		let window = web_sys::window().expect("no global `window` exists");
		let document = window.document().expect("should have a document on window");

		let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(png_data.as_slice()));
		let blob_options = BlobPropertyBag::new();
		blob_options.set_type("image/png");

		let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &blob_options).unwrap();
		let url = Url::create_object_url_with_blob(&blob).unwrap();

		let anchor = document
			.create_element("a")
			.unwrap()
			.dyn_into::<HtmlAnchorElement>()
			.unwrap();

		anchor.set_href(&url);
		anchor.set_download(&format!("{}.png", self.safe_filename()));
		anchor.set_attribute("style", "display: none;").unwrap();

		let body = document.body().expect("should have a body");
		body.append_child(&anchor).unwrap();

		anchor.click();

		body.remove_child(&anchor).unwrap();
		Url::revoke_object_url(&url).unwrap();
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn check_save_png(&mut self, ctx: &Context) {
		self.file_dialog.update(ctx);
		if let Some(path) = self.file_dialog.take_picked() {
			debug!("saving png to {}", path.display());
			self.icongen
				.image()
				.save_with_format(&path, image::ImageFormat::Png)
				.expect("failed to save");
		}
	}

	fn safe_filename(&self) -> String {
		let mut sanitized = Vec::<String>::with_capacity(self.text.len());
		for line in &self.text {
			sanitized.push(
				line.trim()
					.chars()
					.filter_map(|c| match c {
						'/' | ' ' => Some('_'),
						_ if c.is_alphanumeric() => Some(c),
						_ => None,
					})
					.collect::<String>(),
			);
		}
		sanitized.join("_")
	}
}

impl App for AppDoohickey {
	fn save(&mut self, storage: &mut dyn Storage) {
		debug!("saving");
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let mut should_regen_texture = false;
			ui.group(|ui| {
				ui.horizontal(|ui| {
					if ui.button("Save PNG").clicked() {
						self.save_png();
					}
					if self.text.len() < 5 && ui.button("+").clicked() {
						self.text.push(String::new());
					}
				});
				let mut to_remove = None;
				egui::Grid::new("text").num_columns(2).show(ui, |ui| {
					for (idx, line) in self.text.iter_mut().enumerate() {
						ui.horizontal(|ui| {
							if ui.add(TextEdit::singleline(line).char_limit(8)).changed() {
								should_regen_texture = true;
							}
							if idx > 0 && ui.button("-").clicked() {
								to_remove = Some(idx);
								should_regen_texture = true;
							}
						});
						ui.end_row();
					}
				});

				if let Some(idx) = to_remove {
					self.text.remove(idx);
				}
			});
			ui.group(|ui| {
				egui::Grid::new("colors").num_columns(3).show(ui, |ui| {
					ui.label("Background");
					if ui
						.color_edit_button_srgba_unmultiplied(&mut self.icongen.background_color)
						.changed()
					{
						should_regen_texture = true;
					}
					if ui.button("Reset").clicked() {
						self.icongen.background_color = [255, 115, 51, 120];
						should_regen_texture = true;
					}
					ui.end_row();

					ui.label("Text Background");
					if ui
						.color_edit_button_srgba_unmultiplied(
							&mut self.icongen.text_background_color,
						)
						.changed()
					{
						should_regen_texture = true;
					}
					if ui.button("Reset").clicked() {
						self.icongen.text_background_color = [255, 115, 51, 255];
						should_regen_texture = true;
					}
					ui.end_row();

					ui.label("Text");
					if ui
						.color_edit_button_srgba_unmultiplied(&mut self.icongen.text_color)
						.changed()
					{
						should_regen_texture = true;
					}
					if ui.button("Reset").clicked() {
						self.icongen.text_color = [0, 0, 0, 255];
						should_regen_texture = true;
					}
					ui.end_row();
				});
			});

			if should_regen_texture {
				debug!("updating texture");
				self.update_texture(ctx);
			}

			self.icongen.show(ui);
		});

		#[cfg(not(target_arch = "wasm32"))]
		self.check_save_png(ctx);
	}
}
