extern crate darkreign;
extern crate clap;

use std::fs::{File, create_dir};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str;

use clap::{App, Arg};
use darkreign::pack::PackFile;

fn main() {
    let matches = App::new("FTG file extractor")
                          .version("1.0")
                          .author("Karl Hobley <karl@kaed.uk>")
                          .about("Extracts FTG files")
                          .arg(Arg::with_name("ARCHIVE")
                               .help("The archive to extract")
                               .index(1)
                               .required(true))
                          .arg(Arg::with_name("OUT_DIR")
                               .help("Sets the output folder to extract to (defaults to archive's filename in current directory)")
                               .index(2)
                               .required(false))
                          .get_matches();

    let archive_file_name = PathBuf::from(matches.value_of("ARCHIVE").unwrap());
    let output_directory_name = match matches.value_of("OUT_DIR") {
        Some(outdir) => PathBuf::from(outdir),
        None => {
            let mut outdir = archive_file_name.clone();
            outdir.set_extension("");
            outdir
        }
    };

    println!("Extracting {} into {}", archive_file_name.to_str().unwrap(), output_directory_name.to_str().unwrap());

    if output_directory_name.exists() {
        println!("Output directory already exists");
        return;
    }

    if !archive_file_name.is_file() {
        println!("Input file does not exist");
        return;
    }

    if let Err(e) = create_dir(&output_directory_name) {
        println!("Unable to create output directory: {}", e);
        return;
    }

    match File::open(archive_file_name) {
        Ok(mut file) => {
            let mut data = vec![];
            file.read_to_end(&mut data).unwrap();

            match PackFile::new(data.into_boxed_slice()) {
                Ok(pakfile) => {
                    for entry in pakfile.iter_entries() {
                        let name = match str::from_utf8(entry.name()) {
                            Ok(name) => name,
                            Err(_) => {
                                println!("Unable to extract file as its name is not valid UTF-8");
                                continue;
                            }
                        };

                        println!("Extracting {}", name);
                        let mut output_file_name = output_directory_name.clone();
                        output_file_name.push(name);

                        match File::create(output_file_name) {
                            Ok(mut output_file) => {
                                output_file.write_all(entry.data()).unwrap();
                            }
                            Err(e) => {
                                println!("Unable to create output file: {}", e);
                                return;
                            }
                        }
                    }
                }
                Err(e) => println!("Error loading PAK file: {}", e),
            }
        }
        Err(e) => println!("Error opening input file: {}", e),
    }
}
