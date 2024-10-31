use std::fs;

#[derive(Debug)]
struct Size {
    name: String,
    size: u64,
    is_file: bool,
}

fn main() -> std::io::Result<()> {
    // collect args
    let args: Vec<_> = std::env::args().collect();

    // last arg is the path to scan
    let path = &args[args.len() - 1];

    //result
    let sizes = folder_size(path)?;

    //display result somewhere
    size_display(&sizes);

    Ok(())
}

fn folder_size(path: &str) -> std::io::Result<Vec<Size>> {
    // read thr content of the folder, needs to be unwrapped
    let folder_content_w = fs::read_dir(path)?;
    //unwrap and collect the iterator
    let folder_content: Vec<_> = folder_content_w.map(|c| c.unwrap()).collect();

    // retun array
    let mut sizes: Vec<Size> = Vec::new();

    //scan each file/folder
    for content in folder_content.iter() {
        //get the metadata
        let mdata = content.metadata().unwrap();

        if mdata.is_file() {
            //get file name
            let name = content.file_name().into_string().unwrap();
            //get size
            let size = mdata.len();
            let is_file = true;

            sizes.push(Size {
                name,
                size,
                is_file,
            });
        } else {
            let f_name = content.file_name().into_string().unwrap();
            let folder_path = format!("{}/{}", path, &f_name);
            let f_size = folder_size(&folder_path)?;
            let size = f_size.iter().map(|x| x.size).sum();

            sizes.push(Size {
                name: f_name,
                size,
                is_file: false,
            });
        }
    }

    Ok(sizes)
}

fn size_display(sizes: &[Size]) {
    sizes.iter().for_each(|x| {
        println!("{:?}", x);
    });
}
