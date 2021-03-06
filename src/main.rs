use std::{ffi::OsStr, fs, io::BufWriter, path::PathBuf};

#[deny(clippy::pedantic)]

fn main() {
    let path = if let Some(path) = std::env::args().nth(1) {
        path
    } else {
        eprintln!("Usage: gif-extract <gif path>");
        return;
    };

    let path = PathBuf::from(path);
    let file = match fs::File::open(&path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = match options.read_info(file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error decoding file: {}", e);
            return;
        }
    };

    let mut out_dir_name = path
        .file_stem()
        .unwrap_or_else(|| OsStr::new("unnamed"))
        .to_owned();

    out_dir_name.push(OsStr::new("-out"));

    let out_dir = path
        .parent()
        .expect("We already know it's a file inside a dir")
        .join(out_dir_name);

    if let Err(e) = std::fs::create_dir(&out_dir) {
        eprintln!("Error creating output directory: {}", e);
        return;
    }

    let mut frames = 0;
    loop {
        match decoder.read_next_frame() {
            Ok(frame) => {
                if let Some(frame) = frame {
                    let filename = out_dir.join(format!("{}-{}ms.png", frames, frame.delay));
                    let file = match fs::File::create(filename) {
                        Ok(file) => file,
                        Err(e) => {
                            eprintln!("Error creating file for frame {}: {}", frames, e);
                            return;
                        }
                    };
                    let w = BufWriter::new(file);

                    let (width, height) = (u32::from(frame.width), u32::from(frame.height));
                    let mut encoder = png::Encoder::new(w, width, height);

                    encoder.set_color(png::ColorType::RGBA);
                    encoder.set_depth(png::BitDepth::Eight);

                    let mut writer = encoder.write_header().unwrap(); // TODO: are these unwraps fine?
                    writer.write_image_data(&frame.buffer).unwrap();

                    frames += 1;
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error decoding frame {}: {}", frames, e);
                return;
            }
        }
    }

    eprintln!("Saved {} frames to {:?}", frames, &out_dir);
}
