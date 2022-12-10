use std::error::Error;
use std::fs;
use std::process::*;

pub fn write_png(
    dir_img: String,
    img_path: String,
    csv_name: String,
) -> Result<(), Box<dyn Error>> {
    // create directory
    fs::create_dir_all(dir_img.clone()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let png_name: String = format!("{}", img_path);
    let csv_name: String = format!("{}", csv_name);

    Command::new("gnuplot")
        .arg("-e")
        .arg(r#"set terminal png;"#)
        .arg("-e")
        .arg(r#"set datafile separator ",""#)
        .arg("-e")
        .arg(r#"set ticslevel 0;"#)
        .arg("-e")
        .arg(r#"set dgrid3d 100,100;"#)
        .arg("-e")
        .arg(format!(r#"set zrange [{}:{}]"#, -1.0, 1.0))
        .arg("-e")
        .arg(format!(r#"set output "{}""#, png_name))
        .arg("-e")
        // csv plot
        .arg(format!(r#"splot "{}" u 1:2:3 with lines;"#, csv_name))
        .output()
        // mp4
        .expect("failed to start `ffmpeg`");

    Ok(())
}
