use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    process::Stdio,
};

mod camera;
mod gens;
mod hit;
mod material;
mod ray;
mod render;
mod shapes;
mod vec_help;

fn open_out() -> (String, impl Write) {
    let fname = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./out.png".to_owned());
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&fname)
        .unwrap();
    (fname, BufWriter::new(file))
}

fn view_out(filename: &str) {
    // xdg-open seems to rely on shell?
    std::process::Command::new("xdg-open")
        .arg(filename)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn main() {
    let img = gens::final_rand().into_png();
    let (filename, mut out) = open_out();
    out.write_all(&img).unwrap();
    out.flush().unwrap();
    drop(out);
    view_out(&filename);
    println!("{filename}");
}
