use std::{path::PathBuf, io::{BufWriter, Write}, fs::File, time::Instant, fs, sync::Mutex};
use linya::{Bar, Progress};
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::prelude::{IntoParallelIterator, ParallelIterator, IndexedParallelIterator};
#[path = "f_math.rs"] mod f_math;
use f_math::*;
#[derive(Clone, Copy, Debug)]
pub struct Args {
	pub position: Complex,
	pub zoom: f32,
	pub bail: f32,
	pub limit: usize,
}

pub struct Renderer {
	pub args: Args,
	pub size: (u32, u32),
	pub buffer: Vec<[f32;4]>
}

impl Renderer {
	pub fn new(size: (u32, u32), args: Args) -> Self {
		Self {
			args,
			size,
			buffer: vec![[0f32;4];(size.0 * size.1) as usize]
		}
	}
	pub fn pixel(&self, ) -> [f32;4] {
		[0f32;4]
	}
	pub fn render_mandelbrot(&mut self, samples: usize){
			let progress = Mutex::new(Progress::new());
			let bar: Bar= progress
				.lock()
				.unwrap()
				.bar((self.size.0 * self.size.1) as usize, "");
			let ratio = Complex { r: 1., i: (self.size.1 as f32) / (self.size.0 as f32) };
			let size = Complex { r: self.size.0 as f32, i: self.size.1 as f32 };
			self.buffer.clone().into_par_iter().enumerate().map(|(i, _p): (usize, [f32;4])| -> [f32;4]{
				let mut rng = rand::thread_rng();
				let uniform = Uniform::from(0f32..1f32);
				
				let mut pixel = [0f32;4];
				for _ in 0..samples {
					let offset = Complex {
						r: tri_dist(uniform.sample(&mut rng)),
						i: tri_dist(uniform.sample(&mut rng))
					};
					let p = (Complex {
						r: (((i as u32 % self.size.0) as f32 - (size.r * 0.5) + offset.r - 0.5 ) / size.r) * ratio.i / self.args.zoom,
						i: (((i as u32 / self.size.0) as f32 - (size.i * 0.5) + offset.i - 0.5) / size.i) * ratio.i /  self.args.zoom
					}) +  self.args.position;

					let mut it = 0;
					let mut z = p;
					let c = z;

					while it < self.args.limit && (z.r * z.r + z.i * z.i).sqrt() < 2.0 {
						z = z * z + c;
						it += 1;
					}

					// if it < self.args.limit {
					// 	// let theta = z.r.atan2(z.i) + std::f32::consts::PI;
					// 	// // let r = theta.cos();
					// 	// // // let b = theta.sin();
						// let g = (r + b) / 2.0;
					// 
						// pixel[0]+= r.abs();
						// pixel[1]+= g.abs();
						// pixel[2]+= b.abs();
					// }
					// let v =( ((2f32.powf( ((it as f32) / (self.args.limit as f32)) * 32.).cos() / std::f32::consts::PI) + 1.) / 2.) ;

					pixel[0] += (it % 2) as f32;
					pixel[1] += (it % 2) as f32;
					pixel[2] += (it % 2) as f32;
					// pixel[0] += v;
					// pixel[1] += v;
					// // pixel[2] += v;
					// pixel[3] += v;
				}
				
				pixel[0] = (1. - (-(pixel[0] * 2.2 / samples as f32).powf(2.2)).exp2()).clamp(0.,1.);
				pixel[1] = (1. - (-(pixel[1] * 2.2 / samples as f32).powf(2.2)).exp2()).clamp(0.,1.);
				pixel[2] = (1. - (-(pixel[2] * 2.2 / samples as f32).powf(2.2)).exp2()).clamp(0.,1.);
				pixel[3] = (1. - (-(pixel[3] * 2.2 / samples as f32).powf(2.2)).exp2()).clamp(0.,1.);
				pixel[3] = 1.;
				if i % 32 == 0 {
					 progress.lock().unwrap().inc_and_draw(&bar, 32);
				}
				pixel
			}).collect_into_vec(&mut self.buffer) ;
	}
	pub fn export(&self, name: Option<&str>) {
		let mut path = PathBuf::from(format!(
			"{}/", if let Some(f) = name {f}else{"fractal"}
		));
		let mut it = 0;
		while path.exists() {
			path = PathBuf::from(format!("{}_{}/", if let Some(f) = name{f} else {"Fractal"}, it));
			it+=1;
		}
		println!("{}", path.to_str().unwrap());
		let file = if !path.exists() {
			fs::create_dir_all(&path).unwrap();
			path.extend(["image.png"]);
			File::create(path).unwrap()
		} else {
			path.extend(["image.png"]);
			File::open(path).unwrap()
		};
		let writ = &mut BufWriter::new(file);
		let mut encoder = png::Encoder::new(writ, self.size.0, self.size.1);
		encoder.set_color(png::ColorType::Rgba);
		encoder.set_depth(png::BitDepth::Sixteen);
		let mut writer = encoder.write_header().unwrap();
		let now = Instant::now();
		let mut test = writer.stream_writer_with_size(64).unwrap();
		let mut wrote = 0;

		for px in self.buffer.iter() {
			let c = [
				(px[0] * 65535.) as u16,
				(px[1] * 65535.) as u16,
				(px[2] * 65535.) as u16,
				(px[3] * 65535.) as u16,
			];
			let mut arr = vec![0; 0];
			arr.extend([(c[0] >> 8) as u8, c[0] as u8]);
			arr.extend([(c[1] >> 8) as u8, c[1] as u8]);
			arr.extend([(c[2] >> 8) as u8, c[2] as u8]);
			arr.extend([u8::MAX, u8::MAX]);
			wrote += test.write(&arr).unwrap();
		}

		println!(
			"Wrote {} bytes in {:4.4} seconds",
			wrote,
			now.elapsed().as_secs_f32()
		);
	}
}

fn main() {
 let args = Args {bail: 16., limit: 1024, position: Complex {r:-0.7, i:0.}, zoom: 0.35};
 let mut renderer = Renderer::new((1024,1024), args);
 renderer.render_mandelbrot(128);
	renderer.export(None) ;

}
