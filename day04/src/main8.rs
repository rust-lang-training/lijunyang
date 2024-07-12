use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*};
use std::{
    panic,
    process::{Command, Stdio},
};

use std::os::unix;
use std::path::Path;
fn main() {
    // test1();
    // test2();
    // test3();
    // let a = cat("src/hello.txt".as_ref());
    // println!("{:?}", a.unwrap());

    // echo("hello world", "src/hello01.txt".as_ref()).unwrap();
    // touch("src/hello02.txt".as_ref()).unwrap();
    test();
}

fn test() {
    // println!("mkdir a");
    // match fs::create_dir("a") {
    //     Err(e) => println!("{:?}", e.kind()),
    //     Ok(_) => println!("success"),
    // }

    // println!("echo hello > a/b.txt");
    // echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|e| {
    //     println!("{:?}", e.kind());
    // });

    // println!("mkdir -p a/b/c");
    // fs::create_dir_all("a/b/c").unwrap_or_else(|e| {
    //     println!("{:?}", e.kind());
    // });

    // println!("touch a/c/e.txt");
    // touch(&Path::new("a/c/e.txt")).unwrap_or_else(|e| {
    //     println!("{:?}", e.kind());
    // });

    // println!("ls a");
    // match fs::read_dir("a") {
    //     Err(e) => println!("{:?}", e.kind()),
    //     Ok(v) => {
    //         for a in v {
    //             println!("{:?}", a.unwrap().path());
    //         }
    //     }
    // }

    // println!("cat a/c/e.txt");
    // match cat(&Path::new("a/c/e.txt")) {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => println!("{:?}", e.kind()),
    // }

    // println!("rm a/c/e.txt");
    // fs::remove_file("a/c/e.txt").unwrap_or_else(|e| {
    //     println!("{:?}", e.kind());
    // });

    // println!("rmdir a/c");

    // fs::remove_dir("a/c").unwrap_or_else(|e| {
    //     println!("{:?}", e.kind());
    // });
}
fn touch(p: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(p) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn echo(s: &str, p: &Path) -> io::Result<()> {
    let mut f = File::create(p)?;
    f.write_all(s.as_bytes())
}
fn test3() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let res = child.wait().unwrap();
    println!("{:?}", res);
}

fn test2() {
    static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog";
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => panic!("couldn't spawn wc: {:?}", e),
    };
    println!("{:?}", PANGRAM.as_bytes());
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(e) => panic!("couldn't write to wc stdin: {:?}", e),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(e) => panic!("couldn't read wc stdout: {:?}", e),
        Ok(a) => println!("wc responded with: {} {}", s, a),
    }
}
fn test1() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was: \n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was: \n{}", s);
    }
}
