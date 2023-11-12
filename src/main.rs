mod vector;

// use gnuplot::{AutoOption::Fix, AxesCommon, Figure};
use vector::{AngleToVector, Vector, Length};

const G: f64 = 6.6743e-11;
const M_EARTH: f64 = 5.972e24;
const R_EARTH: f64 = 6.3710e6;
const RR_EARTH: f64 = R_EARTH * R_EARTH;
const CIRCUM_EARTH: f64 = std::f64::consts::PI * 2.0 * R_EARTH;
const AV_EARTH: f64 = 2.0 * std::f64::consts::PI / (24.0 * 60.0 * 60.0); // radians/seconds

const TIME_STEP: f64 = 1e-5; // secconds per step

fn main() {
	let mut a_earth = 0.0;
	let initial = Vector(0.0, R_EARTH);
	let mut p = initial; // meters
	let mut v = Vector(AV_EARTH.sin() * R_EARTH, 484.18); // meters/second

	let mut iteration: usize = 0;
	let mut max_d: f64 = 0.0;

	// let mut xs = vec![0.0];
	// let mut ys = vec![0.0];
	// let mut cxs = vec![0.0];
	// let mut cys = vec![0.0];

	while p.0 * p.0 + p.1 * p.1 >= RR_EARTH {
		a_earth += AV_EARTH * TIME_STEP;
		iteration += 1;

		p += v * TIME_STEP;

		let d_squared = p.length_squared();
		let d = d_squared.sqrt();
		max_d = max_d.max(d - R_EARTH);

		let g = G * M_EARTH / d_squared;
		let a = Vector(p.0 / d, p.1 / d);
		v += a * -g * TIME_STEP;

		// if iteration % (0.1 / TIME_STEP).floor() as usize == 0 {
		// 	let relative = Vector(p.0 - initial.0, p.1 - initial.1);

		// 	xs.push(relative.0);
		// 	ys.push(relative.1);
		// 	cxs.push(R_EARTH * a_earth.sin() - initial.0);
		// 	cys.push(R_EARTH * a_earth.cos() - initial.1);
		// }
	}

	let relative = p - initial;

	// xs.push(relative.0);
	// ys.push(relative.1);
	// cxs.push(R_EARTH * a_earth.sin() - initial.0);
	// cys.push(R_EARTH * a_earth.cos() - initial.1);

	// plot_data(xs, ys, cxs, cys);

	let target = a_earth.angle_to_vector() * R_EARTH;
	let off_target = p - target;
	println!("    off target by: {:.3}", off_target.length());
	println!("traveled by objec: {:.3}", relative.length());
	println!("traveled by earth: {:.3}", CIRCUM_EARTH * a_earth);
	let time = TIME_STEP * iteration as f64;
	println!(
		"time to collision: {:.3} s, {:.3} m, {:.3} h",
		time,
		time / 60.0,
		time / 3600.0
	);
	println!("     max distance: {:.3}", max_d);
}

// fn plot_data(xs: Vec<f64>, ys: Vec<f64>, cxs: Vec<f64>, cys: Vec<f64>) {
// 	let mut fg = Figure::new();
// 	fg.axes2d()
// 		.set_aspect_ratio(Fix(-1.0))
// 		.lines(&xs, &ys, &[])
// 		.lines(&cxs, &cys, &[])
// 		.points(&[xs[xs.len() - 1]], &[ys[ys.len() - 1]], &[])
// 		.points(&[cxs[cxs.len() - 1]], &[cys[cys.len() - 1]], &[]);
// 	let _ = fg.show();
// 	// let _ = fg.save_to_pdf("12km.pdf", 20.0, 10.0);
// }
