use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Include shader files in the binary
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("shaders");
    fs::create_dir_all(&dest_path).unwrap();
    
    // Copy shader files
    fs::copy("src/shaders/vertex.glsl", dest_path.join("vertex.glsl")).unwrap();
    fs::copy("src/shaders/fragment.glsl", dest_path.join("fragment.glsl")).unwrap();
    
    println!("cargo:rerun-if-changed=src/shaders/vertex.glsl");
    println!("cargo:rerun-if-changed=src/shaders/fragment.glsl");
}