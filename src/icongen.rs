use crate::font::FONT;
use eframe::egui::{self, ColorImage, TextureHandle};
use egui::{vec2, Image};
use itertools::iproduct;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IconGenerator {
	pub background_color: [u8; 4],
	pub text_background_color: [u8; 4],
	pub text_color: [u8; 4],
	#[serde(skip)]
	texture: Option<TextureHandle>,
}

impl IconGenerator {
	pub fn new(
		background_color: [u8; 4],
		text_background_color: [u8; 4],
		text_color: [u8; 4],
	) -> Self {
		Self {
			background_color,
			text_background_color,
			text_color,
			texture: None,
		}
	}

	fn get_line_pixel_width(&self, text: &str) -> u32 {
		let mut line_width = 0;
		for c in text.chars() {
			if let Some(char_data) = FONT.get(&c.to_ascii_uppercase()) {
				line_width += char_data.width() + 1;
			}
		}
		line_width.saturating_sub(1)
	}

	pub fn generate_image(&self, text: &[String]) -> ColorImage {
		const SIZE: u32 = 32;
		const CENTER: u32 = SIZE / 2;

		let mut pixels = vec![[0u8; 4]; SIZE.pow(2) as usize];

		// Fill background
		for pixel in pixels.iter_mut() {
			*pixel = self.background_color;
		}

		let line_count_half = text.len() / 2;

		for (line_number, line) in text.iter().enumerate() {
			let line_width = self.get_line_pixel_width(line);

			// Calculate starting position for this line
			let mut pos_y = CENTER.saturating_sub(6 * line_count_half as u32) + 1;
			if text.len() % 2 != 0 {
				pos_y = pos_y.saturating_sub(3);
			}
			pos_y = pos_y.saturating_add(6 * line_number as u32);

			let start_x = if line_width < CENTER * 2 {
				CENTER.saturating_sub((line_width + 1) / 2)
			} else {
				0
			};

			// Draw text background
			let ys = (pos_y.saturating_sub(1)..=pos_y.saturating_add(5))
				.filter(|y| (0..SIZE).contains(y));
			let xs = (start_x.saturating_sub(1)..=start_x.saturating_add(line_width))
				.filter(|x| (0..SIZE).contains(x));
			for (y, x) in iproduct!(ys, xs) {
				pixels[(y * SIZE + x) as usize] = self.text_background_color;
			}

			// Draw text
			let mut current_x = start_x;
			for char_data in line
				.chars()
				.filter_map(|c| FONT.get(&c.to_ascii_uppercase()))
			{
				for (i, &pixel) in char_data.pixels().iter().enumerate() {
					if pixel == 1 {
						let char_x = i as u32 % char_data.width();
						let char_y = i as u32 / char_data.width();

						let x = current_x.saturating_add(char_x);
						let y = pos_y.saturating_add(char_y);

						if x < SIZE && y < SIZE {
							pixels[(y * SIZE + x) as usize] = self.text_color;
						}
					}
				}
				current_x = current_x.saturating_add(char_data.width() + 1);
			}
		}

		ColorImage::from_rgba_unmultiplied(
			[SIZE as usize, SIZE as usize],
			&pixels
				.iter()
				.flat_map(|p| p.iter())
				.copied()
				.collect::<Vec<u8>>(),
		)
	}

	pub fn update_texture(&mut self, ctx: &egui::Context, text: &[String]) {
		let image = self.generate_image(text);
		self.texture = Some(ctx.load_texture("icon", image, egui::TextureOptions::NEAREST));
	}

	pub fn show(&self, ui: &mut egui::Ui) {
		if let Some(texture) = &self.texture {
			ui.add(Image::new(texture).fit_to_exact_size(vec2(256., 256.)));
		}
	}
}

/*
#[derive(Default)]
struct IconGenApp {
	generator: Option<IconGenerator>,
	text: Vec<String>,
}

impl eframe::App for IconGenApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if self.generator.is_none() {
			let background_color = [255, 115, 51, 120];
			let text_background_color = [255, 115, 51, 255];
			let text_color = [0, 0, 0, 255];

			self.generator = Some(IconGenerator::new(ctx, background_color, text_background_color, text_color));
			self.text = vec![
				"123".to_string(),
				"456".to_string(),
				"789".to_string(),
				"0".to_string(),
			];
		}

		egui::CentralPanel::default().show(ctx, |ui| {
			let generator = self.generator.as_mut().unwrap();

			let mut text_changed = false;
			for line in &mut self.text {
				if ui.text_edit_singleline(line).changed() {
					text_changed = true;
				}
			}

			if text_changed {
				generator.update_texture(ctx, &self.text);
			}

			generator.show(ui);
		});
	}
}

fn main() -> eframe::Result<()> {
	let native_options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(320.0, 240.0)),
		..Default::default()
	};

	eframe::run_native(
		"Icon Generator",
		native_options,
		Box::new(|cc| Box::new(IconGenApp::default()))
	)
}
*/
