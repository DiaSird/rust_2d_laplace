use std::fs;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

const NX: usize = 100; // x-direction splitted number
const NY: usize = 100; // y-direction splitted number

pub fn write_csv(dir_path: String, file_path: String, u: [[f64; 100]; 100]) -> Result<(), Box<dyn error::Error>> {
    // create directory
    fs::create_dir_all(dir_path).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let file = File::create(file_path).unwrap();

    println!("{:?}", file);

    let mut w = BufWriter::new(file);
    write!(w, "x,y,u\n").unwrap();

    for i in 0..NX {
        for j in 0..NY {
            let s = format!("{},{},{}\n", i, j, u[i][j],);
            write!(w, "{}", s).unwrap();
        }
    }
    w.flush().unwrap();
    Ok(())
}
