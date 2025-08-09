use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ico_file>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    
    // 检查 ICO 文件头
    if buffer[0] == 0 && buffer[1] == 0 && buffer[2] == 1 && buffer[3] == 0 {
        println!("{} is a valid ICO file", file_path);
    } else {
        println!("{} is not a valid ICO file", file_path);
        println!("Header: {:?}", buffer);
    }
    
    Ok(())
}