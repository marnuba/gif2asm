use std::{fs::File, process::exit, path::PathBuf};
use gif::DecodeOptions;
use clap::Parser;
use std::path::Path;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {

    infile: String,

    //#[arg(short, long)]
    //outfile: Option<String>,

    //#[arg(short, long)]
    //palette_file: Option<String>,

    //#[arg(short, long)]
    //assembler: Option<String>,

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

    
    let mut opts = DecodeOptions::new();

    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = opts.read_info(input).unwrap();

    if let Some(file) = &mut asmfile {
        write!(file, "\n{}_clut:\n", label)
        .expect("error writing to asm file");
    }

    // write palette
    if let Some(pal) = decoder.global_palette() {
        
        // write #entries in palette
        if let Some(file) = &mut clutfile {
            file.write_all(&[pal.len() as u8]).expect("error writing to clut file");
        }

        if let Some(file) = &mut asmfile {
            write!(file, ".byte {:02X}", pal.len())
                .expect("error writing to asm file");
        }

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

    if let Some(file) = &mut asmfile {
        write!(file, "\n{}_img:\n", label)
        .expect("error writing to asm file");
    }
    
    // write image data
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        for h in 0..frame.height {
            if let Some(file) = &mut asmfile {
                write!(file, ".byte").expect("error writing to asm file");
                for w in 0..frame.width-1 {
                    let ix = h as usize * frame.width as usize + w as usize;
                    write!(file, " ${:02X},", frame.buffer[ix as usize])
                        .expect("error writing to asm file");
                }
                write!(file, " ${:02X}\n", frame.buffer[(h+1) as usize * (frame.width-1) as usize])
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

