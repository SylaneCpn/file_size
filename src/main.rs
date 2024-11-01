use clap::Parser;
use std::fs;

/// Simple program to list the size of the files in the specified path
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Print mode : 0 => terminal, 1 => file
    #[arg(short, long, default_value_t = 0)]
    mode: u8,
    /// Path to scan
    path: String,
}

#[derive(Debug)]
struct Size {
    name: String,
    size: u64,
    is_file: bool,
}

fn main() -> std::io::Result<()> {
    // collect args
    let args = Cli::parse();

    let mode = args.mode;

    // last arg is the path to scan
    let path = args.path;

    //result
    let mut sizes = folder_size(path)?;

    //display result somewhere
    size_display(&mut sizes, mode)?;

    Ok(())
}

fn folder_size(path: String) -> std::io::Result<Vec<Size>> {
    // read thr content of the folder, needs to be unwrapped
    // retun array
    let path = path.clone();
    let handle = std::thread::spawn(move || -> std::io::Result<Vec<Size>> {
        let mut sizes: Vec<Size> = Vec::new();
        if let Ok(folder_content_w) = fs::read_dir(&path) {
            //unwrap and collect the iterator
            let folder_content: Vec<_> = folder_content_w.map(|c| c.unwrap()).collect();

            //scan each file/folder
            for content in folder_content.iter() {
                //get the metadata
                let mdata = content.metadata().unwrap();

                let name = content.file_name().into_string().unwrap();
                let size;
                let is_file;

                if mdata.is_file() {
                    //get size
                    size = mdata.len();
                    is_file = true;
                } else {
                    let folder_path = format!("{}/{}", path, &name);
                    let f_size = folder_size(folder_path)?;

                    size = f_size.iter().map(|x| x.size).sum();
                    is_file = false;
                }

                sizes.push(Size {
                    name,
                    size,
                    is_file,
                });
            }
        }
        Ok(sizes)
    });

    let result = handle.join().unwrap()?;
    Ok(result)
}

fn size_display(sizes: &mut [Size], mode: u8) -> std::io::Result<()> {
    sizes.sort_by_key(|k| k.size);
    let mut to_write = String::new();

    sizes.iter().for_each(|x| {
        to_write = format!(
            "{}{}\t{}\t{}\n",
            to_write,
            if x.is_file { "F" } else { "DIR" },
            &x.name,
            size_unit(x.size)
        );
    });

    if mode == 0 {
        print!("{}", to_write);
    } else {
        fs::write("size.txt", to_write.as_bytes())?;
    }

    Ok(())
}

fn size_unit(size: u64) -> String {
    let mut dept = 0;
    let original_size = size as f64;
    let mut s = original_size;

    loop {
        if s / 1000.0 >= 1.0 {
            s /= 1000.0;
            dept = dept + 1;
        } else {
            break;
        }
    }

    if dept == 0 {
        format!("{} b", original_size)
    } else if dept == 1 {
        format!("{} Kb", original_size / 1_000.0)
    } else if dept == 2 {
        format!("{} Mb", original_size / 1_000_000.0)
    } else if dept == 3 {
        format!("{} Gb", original_size / 1_000_000_000.0)
    } else {
        format!("{} Tb", original_size / 1_000_000_000_000.0)
    }
}
