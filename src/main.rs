use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        if args[2] == "-p" {
            let hash = &args[3];
            let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
            let data = fs::read(path).unwrap();
            let mut obj = ZlibDecoder::new(&data[..]);
            let mut s = String::new();
            obj.read_to_string(&mut s).unwrap();
            let output = s.split_off(s.find('\0').unwrap() + 1);
            print!("{}", output)
        } else {
            println!("unknown flag: {}", args[2])
        }
    }
}
