/*-------------------------------------------------
Solution of the 2D Laplace equation
Reference:
https://www.ohmsha.co.jp/book/9784274221705/
-------------------------------------------------*/
use laplace_eq::module::*;
use std::f64::consts::PI;
use std::{time::Instant, *};

// Input
const LIM: usize = 1000; // maximum repeated number
const NX: usize = 100; // x-direction splitted number
const NY: usize = 100; // y-direction splitted number

// file path
const DIR_PATH: &'static str = "rst";
const FILE_PATH: &'static str = "rst/rst_u.csv";
const DIR_IMG: &str = DIR_PATH;
const IMG_PATH: &str = "rst/2d_laplace.png";
const IMG_PATH2: &str = "rst/2d_laplace2.png";

pub fn main() {
    let start = Instant::now();

    let dir_path: String = format!("{}", DIR_PATH);
    let file_path: String = format!("{}", FILE_PATH);
    let dir_img: String = format!("{}", DIR_IMG);
    let img_path: String = format!("{}", IMG_PATH);
    let img_path2: String = format!("{}", IMG_PATH2);

    // Initialize
    let mut u: [[f64; NX]; NY] = [[0.0; NX]; NY];

    for j in 0..NY {
        // u_0,j = sin(2πj/100)
        let temp = (2.0 * PI * j as f64 / NY as f64).sin();
        u[0][j] = temp;
    }

    // Jacobi method
    for _i in 0..LIM {
        u = iteration::iteration(u);
    }

    // Output
    // println!("{:?}", u);
    out_csv::write_csv(dir_path, file_path.clone(), u).unwrap();

    // gif test
    // draw_pdf::plotters().unwrap();

    // drawing
    gnuplot::write_png(dir_img.clone(), img_path, file_path).unwrap();
    plot::plot(dir_img.clone(), img_path2, u).unwrap();
    // plot_gif::plot(dir_img, u).unwrap();

    // CPU time
    cpu_time::time(start);
}
