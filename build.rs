use std::env;
use std::fs::{ReadDir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn write_files(output: &mut File, root: &Path, dir: ReadDir) {
    for entry in dir {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let path = entry.path();
        if file_type.is_file() {
            let name = path.strip_prefix(root).unwrap();
            println!("Write file {} {}", name.display(), path.display());
            writeln!(output, "    \"{}\" => Some(include_bytes!(\"{}\")),",
                name.display(), path.display()).unwrap();
        } else if file_type.is_dir() {
            write_files(output, root, path.read_dir().unwrap());
        }
    }
}

fn main() {
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let site = src.join("site/_site");
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut output = File::create(dst.join("assets.rs")).unwrap();

    println!();
    writeln!(output, "pub fn get(name: &str) -> Option<&'static [u8]> {{").unwrap();
    writeln!(output, "  match name {{").unwrap();
    write_files(&mut output, &src, site.read_dir().unwrap());
    writeln!(output, "    _ => None").unwrap();
    writeln!(output, "  }}").unwrap();
    writeln!(output, "}}").unwrap();
}
