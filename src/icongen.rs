use eframe::egui::{self, ColorImage, TextureHandle};
use egui::{Color32, Image, vec2};
use image::{ImageFormat, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::LazyLock};

pub const CHAR_WIDTH: u32 = 3;
pub const CHAR_HEIGHT: u32 = 5;

fn load_spritesheet() -> HashMap<char, RgbaImage> {
	// because i'm silly and like to overcomplicate things, the font is a QOI
	// spritesheet, except there's extra data appended to the end!
	// the characters associated with each sprite (they are 3x5 each), followed by
	// how many characters there are.

	let spritesheet_file = include_bytes!("spritesheet.qoi");
	let file_len = spritesheet_file.len();
	let character_amount = spritesheet_file[file_len - 1] as usize;
	let characters = (0..character_amount)
		.rev()
		.map(|idx| spritesheet_file[file_len - (idx + 2)] as char)
		.collect::<Vec<char>>();

	let spritesheet = image::load_from_memory_with_format(
		&spritesheet_file[..file_len - (character_amount + 1)],
		ImageFormat::Qoi,
	)
	.unwrap()
	.into_rgba8();

	let mut sprites = HashMap::with_capacity(characters.len());
	for (idx, character) in characters.iter().enumerate() {
		let cropped = image::imageops::crop_imm(
			&spritesheet,
			(CHAR_WIDTH + 1) * (idx as u32),
			0,
			CHAR_WIDTH,
			CHAR_HEIGHT,
		);
		sprites.insert(*character, cropped.to_image());
	}

	sprites
}

pub static FONT: LazyLock<HashMap<char, RgbaImage>> = LazyLock::new(load_spritesheet);

fn default_internal_image() -> RgbaImage {
	RgbaImage::new(32, 32)
}

fn default_color_image() -> ColorImage {
	ColorImage::filled([32, 32], Color32::TRANSPARENT)
}

#[derive(Serialize, Deserialize)]
pub struct IconGenerator {
	pub background_color: [u8; 4],
	pub text_background_color: [u8; 4],
	pub text_color: [u8; 4],
	#[serde(skip, default = "default_internal_image")]
	image: RgbaImage,
	#[serde(skip, default = "default_color_image")]
	paint_image: ColorImage,
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
			image: default_internal_image(),
			paint_image: default_color_image(),
			texture: None,
		}
	}

	fn get_line_pixel_width(&self, text: &str) -> u32 {
		let mut line_width = 0;
		for c in text.chars() {
			if c == ' ' {
				line_width += 2
			} else if let Some(char_data) = FONT.get(&c.to_ascii_uppercase()) {
				line_width += char_data.width() + 1;
			}
		}
		line_width.saturating_sub(1)
	}

	pub fn update_image(&mut self, text: &[String]) {
		const SIZE: u32 = 32;
		const CENTER: u32 = SIZE / 2;

		// Fill background
		for pixel in self.image.pixels_mut() {
			*pixel = Rgba(self.background_color);
		}

		let line_count_half = text.len() / 2;

		for (line_number, line) in text.iter().enumerate() {
			let line_width = self.get_line_pixel_width(line);

			// Calculate starting position for this line
			let mut pos_y = CENTER.saturating_sub(6 * line_count_half as u32) + 1;
			if !text.len().is_multiple_of(2) {
				pos_y = pos_y.saturating_sub(3);
			}
			pos_y = pos_y.saturating_add(6 * line_number as u32);

			let start_x = if line_width < SIZE {
				CENTER.saturating_sub(line_width.div_ceil(2))
			} else {
				0
			};

			// Draw text background
			for x in (start_x.saturating_sub(1)..=start_x.saturating_add(line_width))
				.filter(|x| (0..SIZE).contains(x))
			{
				for y in (pos_y.saturating_sub(1)..=pos_y.saturating_add(5))
					.filter(|y| (0..SIZE).contains(y))
				{
					*self.image.get_pixel_mut(x, y) = Rgba(self.text_background_color);
				}
			}

			// Draw text
			let mut current_x = start_x;
			for character in line.chars() {
				if character == ' ' {
					current_x = current_x.saturating_add(2);
					continue;
				}
				let sprite = match FONT.get(&character.to_ascii_uppercase()) {
					Some(sprite) => sprite,
					None => continue,
				};
				for x in 0..sprite.width() {
					for y in 0..sprite.height() {
						// skip any pixels if the alpha is 0
						if sprite.get_pixel(x, y).0[3] != 0 {
							self.image.put_pixel(
								current_x.saturating_add(x),
								pos_y.saturating_add(y),
								Rgba(self.text_color),
							);
						}
					}
				}
				current_x = current_x.saturating_add(CHAR_WIDTH + 1);
			}
		}

		for (idx, pixel) in self.image.pixels().enumerate() {
			self.paint_image.pixels[idx] =
				Color32::from_rgba_unmultiplied(pixel[0], pixel[1], pixel[2], pixel[3]);
		}
	}

	pub fn update_texture(&mut self, ctx: &egui::Context, text: &[String]) {
		self.update_image(text);
		self.texture = Some(ctx.load_texture(
			"icon",
			self.paint_image.clone(),
			egui::TextureOptions::NEAREST,
		));
	}

	pub fn show(&self, ui: &mut egui::Ui) {
		if let Some(texture) = &self.texture {
			ui.add(Image::new(texture).fit_to_exact_size(vec2(256., 256.)));
		}
	}

	#[inline]
	pub fn image(&self) -> &RgbaImage {
		&self.image
	}
}
