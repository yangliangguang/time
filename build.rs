fn main() {
    #[cfg(target_os = "windows")] {
        // Try different icon files in order of preference
        let icon_files = ["resources/clock-icon.ico", "resources/icon.ico"];
        
        for icon_file in &icon_files {
            if std::path::Path::new(icon_file).exists() {
                let mut res = winres::WindowsResource::new();
                match res.set_icon(icon_file).compile() {
                    Ok(_) => {
                        println!("Successfully compiled with icon: {}", icon_file);
                        break;
                    }
                    Err(e) => {
                        println!("Failed to compile with icon {}: {}", icon_file, e);
                        continue;
                    }
                }
            }
        }
    }
    
    slint_build::compile("ui/clock.slint").unwrap();
}