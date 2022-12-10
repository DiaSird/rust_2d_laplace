use plotters::prelude::*;
use std::fs;

const NX: usize = 100; // x-direction splitted number
const NY: usize = 100; // y-direction splitted number

pub fn plot(
    dir_img: String,
    img_path: String,
    u: [[f64; 100]; 100],
) -> Result<(), Box<dyn std::error::Error>> {
    // create directory
    fs::create_dir_all(&dir_img).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // file size
    let size = (640, 480);

    // Bit map
    let root = BitMapBackend::new(&img_path, size).into_drawing_area();

    // white back
    root.fill(&WHITE).unwrap();

    // set axis
    let x_spec = 0.0..NX as f64;
    let y_spec = 0.0..NY as f64;
    let z_spec = -2.0..2.0;

    // caption and font
    let caption = "2D Laplace";
    let font = ("sans-serif", 20);

    // axis specimen
    let mut chart = ChartBuilder::on(&root)
        .margin(30)
        .caption(caption, font)
        .build_cartesian_3d(x_spec, z_spec, y_spec)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    // data point of x and y
    let x = (0..NX).map(|a| a as f64); // i
    let y = (0..NY).map(|b| b as f64); // j

    chart
        .draw_series(
            // plot x, y, u
            SurfaceSeries::xoz(x, y, |x: f64, y: f64| {
                let i = x as usize;
                let j = y as usize;
                // println!("{}", u[i][j]);
                u[i][j]
            })
            .style_func(&|&v| {
                (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v / 5.0, 1.0, 0.7)).into()
            }),
        )
        .unwrap();

    root.present().expect(
        "Unable to write result to file, please make sure 
        'plotters-doc-data' dir exists under current dir",
    );
    // println!("Result has been saved to {}", img_path);
    println!("Result has been saved to {}", &img_path);

    Ok(())
}
