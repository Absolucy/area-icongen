use image::{ImageBuffer, Rgba};
use itertools::iproduct;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Character {
	width: u32,
	pixels: Vec<u8>,
}

pub struct IconGenerator {
	background_color: Rgba<u8>,
	text_background_color: Rgba<u8>,
	text_color: Rgba<u8>,
	font: HashMap<char, Character>,
}

impl IconGenerator {
	pub fn new(
		background_color: Rgba<u8>,
		text_background_color: Rgba<u8>,
		text_color: Rgba<u8>,
	) -> Self {
		let chars = [
			('A', (3, vec![0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1])),
			('B', (3, vec![1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0])),
			('C', (3, vec![0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1])),
			('D', (3, vec![1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0])),
			('E', (3, vec![1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1])),
			('F', (3, vec![1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0])),
			('G', (3, vec![0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1])),
			('H', (3, vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1])),
			('I', (3, vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1])),
			('J', (3, vec![0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0])),
			('K', (3, vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1])),
			('L', (3, vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1])),
			('M', (3, vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1])),
			('N', (3, vec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1])),
			('O', (3, vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0])),
			('P', (3, vec![1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0])),
			('Q', (3, vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1])),
			('R', (3, vec![1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1])),
			('S', (3, vec![0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0])),
			('T', (3, vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0])),
			('U', (3, vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1])),
			('V', (3, vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0])),
			('W', (3, vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1])),
			('X', (3, vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1])),
			('Y', (3, vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0])),
			('Z', (3, vec![1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1])),
			('/', (3, vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0])),
			('.', (3, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0])),
			(' ', (1, vec![0, 0, 0, 0, 0])),
			('1', (3, vec![0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1])),
			('2', (3, vec![1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1])),
			('3', (3, vec![1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1])),
			('4', (3, vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1])),
			('5', (3, vec![1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1])),
			('6', (3, vec![1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1])),
			('7', (3, vec![1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1])),
			('8', (3, vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1])),
			('9', (3, vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1])),
			('0', (3, vec![1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1])),
		];

		let font = chars
			.into_iter()
			.map(|(c, (width, pixels))| (c, Character { width, pixels }))
			.collect();

		Self {
			background_color,
			text_background_color,
			text_color,
			font,
		}
	}

	fn get_line_pixel_width(&self, text: &str) -> u32 {
		let mut line_width = 0;
		for char_data in text
			.chars()
			.filter_map(|c| self.font.get(&c.to_ascii_uppercase()))
		{
			line_width += char_data.width + 1;
		}
		line_width.saturating_sub(1)
	}

	pub fn generate(&self, text: &[String]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
		const WIDTH: u32 = 32;
		const HEIGHT: u32 = 32;
		const CENTER_X: u32 = WIDTH / 2;
		const CENTER_Y: u32 = HEIGHT / 2;

		let mut image = ImageBuffer::new(WIDTH, HEIGHT);

		// Fill background
		for pixel in image.pixels_mut() {
			*pixel = self.background_color;
		}

		let line_count_half = (text.len() / 2) as u32;

		for (line_number, line) in text.iter().enumerate() {
			let line_number = line_number as u32;
			let line_width = self.get_line_pixel_width(line);

			// Calculate starting position for this line
			let mut pos_y = CENTER_Y - (6 * line_count_half) + 1;
			if text.len() % 2 != 0 {
				pos_y -= 3;
			}
			pos_y += 6 * line_number;

			let pos_x = CENTER_X - ((line_width + 1) / 2);

			// Draw text background
			let ys = ((pos_y - 1)..=(pos_y + 5)).filter(|y| (0..HEIGHT).contains(y));
			let xs = ((pos_x - 1)..=(pos_x + line_width)).filter(|x| (0..WIDTH).contains(x));
			for (y, x) in iproduct!(ys, xs) {
				image.put_pixel(x, y, self.text_background_color);
			}

			// Draw text
			let mut current_x = pos_x;
			for char_data in line
				.chars()
				.filter_map(|c| self.font.get(&c.to_ascii_uppercase()))
			{
				for (i, &pixel) in char_data.pixels.iter().enumerate() {
					let char_x = i as u32 % char_data.width;
					let char_y = i as u32 / char_data.width;

					if pixel == 1 {
						let x = current_x + char_x;
						let y = pos_y + char_y;
						if (0..WIDTH).contains(&x) && (0..HEIGHT).contains(&y) {
							image.put_pixel(x, y, self.text_color);
						}
					}
				}
				current_x += char_data.width + 1;
			}
		}

		image
	}
}

/*
fn main() {
	// Convert hex colors to RGBA
	let background_color = [255, 115, 51, 120];
	let text_background_color = [255, 115, 51, 255];
	let text_color = [0, 0, 0, 255]; // #000000

	let text = vec!["Prison".to_string(), "Shower".to_string()];

	let generator = IconGenerator::new(
		Rgba(background_color),
		Rgba(text_background_color),
		Rgba(text_color),
	);
	let image = generator.generate(&text);

	// Save the image
	image.save("icon.png").unwrap();
}
*/
