use std::{fs::File, process::exit, path::PathBuf};
use gif::DecodeOptions;
use clap::Parser;
use std::path::Path;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {
    infile: String,

    #[arg(short, long)]
    bin: bool,

    #[arg(short, long)]
    asm: bool,
}

fn main() {

    let mut args = Args::parse();

    check_filename(&args.infile);

    // if no args are given, default to asm output
    if !args.asm && !args.bin {
        args.asm = true;
    }

    // output file names & label
    let input = File::open(&args.infile)
        .expect("Could not open input file.");
        
    let mut asmpath = PathBuf::from(&args.infile);
    asmpath.set_extension("asm");
    
    let mut clutpath = asmpath.clone();
    clutpath.set_extension("clut");
    
    let mut imgpath = clutpath.clone();
    imgpath.set_extension("img");

    let labelpath = imgpath.clone();
    let label = String::from(labelpath.file_stem().unwrap().to_string_lossy());

    // open files for writing
    let mut asmfile = if args.asm {
        Some(File::create(asmpath)
        .expect("Could not open asm file to write."))
    } else {
        None
    };

    let mut clutfile = if args.bin {
        Some(File::create(clutpath)
            .expect("Could not open asm file to write."))
    } else {
        None
    };

    let mut imgfile = if args.bin {
        Some(File::create(imgpath)
        .expect("Could not open asm file to write."))
    } else {
        None
    };
    
    // decode gif file 
    let mut opts = DecodeOptions::new();

    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = opts.read_info(input).unwrap();

    // write palette
    if let Some(pal) = decoder.global_palette() {

        // write label for clut size
        if let Some(file) = &mut asmfile {
            write!(file, "\n{}_clut_size:\n", label)
            .expect("error writing to asm file");
        }
                
        // write #entries in clut
        let len = ((pal.len() / 3) % 256) as u8;
        if let Some(file) = &mut asmfile {
            write!(file, ".byte {:02X}\n", len)
                .expect("error writing to asm file");
        }

        // write label for clut size
        if let Some(file) = &mut asmfile {
            write!(file, "\n{}_clut:\n", label)
            .expect("error writing to asm file");
        }

        if let Some(file) = &mut clutfile {
            file.write(&[len])
                .expect("error writing to clut file");
        }

        // write palette
        for i in (0..pal.len()).step_by(3) {

            if let Some(file) = &mut asmfile {
                write!(file, ".byte ${:02X}, ${:02X}, ${:02X}, $00\n", 
                    pal[i+2],
                    pal[i+1],
                    pal[i],
                ).expect("error writing to asm file");
            }

            if let Some(file) = &mut clutfile {
                file.write_all(&[pal[i+2], pal[i+1], pal[i], 0x00])
                .expect("error writing to clut file");
            }
        }
    }
    
    // write image data
    while let Some(frame) = decoder.read_next_frame().unwrap() {

        // write label for image size
        if let Some(file) = &mut asmfile {
            write!(file, "\n{}_img_size:\n", label)
            .expect("error writing to asm file");
        }

        // write img dimensions
        if let Some(file) = &mut asmfile {
            write!(file, ".byte {:02X}, {:02X}\n", 
                frame.width % 256,
                frame.width / 256,
            ).expect("error writing to asm file");

            write!(file, ".byte {:02X}, {:02X}\n", 
                frame.height % 256,
                frame.height / 256,
            ).expect("error writing to asm file");
        }

        if let Some(file) = &mut imgfile {
            file.write_all(&[
                (frame.width % 256) as u8,
                (frame.width / 256) as u8,
                (frame.height % 256) as u8,
                (frame.height / 256) as u8,
            ]).expect("error writing to img file");
        }

        // write label for image data
        if let Some(file) = &mut asmfile {
            write!(file, "\n{}_img:\n", label)
            .expect("error writing to asm file");
        }

        for h in 0..frame.height {
            if let Some(file) = &mut asmfile {
                write!(file, ".byte").expect("error writing to asm file");
                for w in 0..frame.width-1 {
                    let ix = h as usize * frame.width as usize + w as usize;
                    write!(file, " ${:02X},", frame.buffer[ix as usize])
                        .expect("error writing to asm file");
                }
                write!(file, " ${:02X}\n", frame.buffer[(h+1) as usize * (frame.width) as usize - 1])
                    .expect("error writing to asm file");
            }            
        }

        if let Some(file) = &mut imgfile {
            file.write_all(&frame.buffer).expect("error writing to img file");
        }
    }
}

fn check_filename(filename: &String) {
    // check: input must be gif
    let filepath = Path::new(filename);

    if !filepath.is_file() {
        println!("Input needs to be a file.");
        exit(-1);
    }

    if let Some(ext) = filepath.extension() {
        let exts = ext.to_string_lossy();
        if exts.to_lowercase() != "gif" {
            println!("input file needs to have the gif extension");
            exit(-1);
        }
    } else {
        println!("Input file does not have a file extension");
        exit(-1);
    }
}

