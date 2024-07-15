use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn copy(read: &mut BufReader<File>, write: &mut BufWriter<File>) -> io::Result<i32> {
    const SIZE: usize = 2;

    let mut buffer: [u8; SIZE] = [0u8; SIZE];
    let mut len = 0;

    loop {
        match read.read(&mut buffer) {
            Ok(0) => return Ok(len),
            Ok(n) => {
                write.write(&buffer[..n])?;
                len += n as i32;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}

pub fn copy_file<P: AsRef<Path>>(source: P, target: P) {
    let a = File::open(source).unwrap();
    println!("{:?}", a);
    let b = File::create(target).unwrap();

    copy(&mut BufReader::new(a), &mut BufWriter::new(b)).unwrap();
}
