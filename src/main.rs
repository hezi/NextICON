use std::env; 
use std::io::{Read, Cursor, Write};
use std::fs::File;
use mach_object::{OFile, MachCommand, LoadCommand};

fn dump_icon(path: &String) {
    let mut f = File::open(&path).unwrap();
    let mut buf = Vec::new();
    let size = f.read_to_end(&mut buf).unwrap();
    let mut cur = Cursor::new(&buf[..size]);
    if let OFile::MachFile { ref header, ref commands } = OFile::parse(&mut cur).unwrap() {    
        assert_eq!(header.ncmds as usize, commands.len());
        for &MachCommand(ref cmd, _cmdsize) in commands {
            if let &LoadCommand::Segment { ref segname, ref sections, .. } = cmd {
                if segname == "__ICON" {
                    for ref sect in sections {
                        if sect.sectname == "__header" {
                            continue;
                        }

                        println!("dumping {}.tiff...", sect.sectname);

                        let offset = sect.offset as usize;
                        let size = offset+sect.size;
                        let part = &buf[offset..size];
                        let mut f = File::create(format!("{}.tiff", sect.sectname)).expect("Unable to create file");
                        f.write_all(part).expect("Unable to write data");
                    }
                }
            }
        }
    }
}

fn help() {
    println!("Usage:
next_icon <path_to_executable>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help()
        },
        2 => {
            dump_icon(&args[1]);
        }
        _ => {
            help()
        }
    }
}
