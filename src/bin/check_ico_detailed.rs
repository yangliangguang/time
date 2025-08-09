use std::env;
use std::fs::File;
use ico::IconDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ico_file>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let icon_dir = IconDir::read(file)?;
    
    println!("{} is a valid ICO file with {} entries", file_path, icon_dir.entries().len());
    for (i, entry) in icon_dir.entries().iter().enumerate() {
        println!("  Entry {}: {}x{}", i, entry.width(), entry.height());
    }
    
    Ok(())
}