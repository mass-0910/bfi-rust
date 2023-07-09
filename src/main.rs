use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter};

fn main() {
    // Commandline argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open file
    let mut fp = File::open(filename).expect("file not found");
    // Read file
    let mut contents = String::new();
    fp.read_to_string(&mut contents)
        .expect("something went wrong reading the file!");
    exec_bf(contents, None);
}

fn exec_bf(bfsrc: String, mut bufstream: Option<BufWriter<&mut Vec<u8>>>) -> [u8; 30000] {
    let mut mem: [u8; 30000] = [0; 30000];
    let mut mem_idx: usize = 0;
    let mut src_idx: usize = 0;
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let bfsrc_chars: Vec<char> = bfsrc.chars().collect();
    while src_idx < bfsrc.len() {
        match bfsrc_chars[src_idx] {
            '>' => {
                mem_idx += 1;
                ()
            }
            '<' => {
                mem_idx -= 1;
                ()
            }
            '+' => {
                mem[mem_idx] += 1;
                ()
            }
            '-' => {
                mem[mem_idx] -= 1;
                ()
            }
            '.' => {
                stdout.write(&[mem[mem_idx] as u8]).expect("error");
                if !bufstream.is_none() {
                    let _ = bufstream.as_mut().unwrap().write(&[mem[mem_idx] as u8]);
                }
                ()
            },
            ',' => {
                let mut buf: [u8; 1] = [0; 1];
                stdin.read_exact(&mut buf).unwrap();
                mem[mem_idx] = buf[0];
                ()
            },
            '[' => {
                if mem[mem_idx] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        src_idx += 1;
                        count += match bfsrc_chars[src_idx] {
                            '[' => 1,
                            ']' => -1,
                            _ => 0
                        };
                    }
                }
                ()
            },
            ']' => {
                if mem[mem_idx] != 0 {
                    let mut count = 1;
                    while count > 0 {
                        src_idx -= 1;
                        count += match bfsrc_chars[src_idx] {
                            '[' => -1,
                            ']' => 1,
                            _ => 0
                        };
                    }
                }
                ()
            },
            _ => {
                ()
            }
        }
        src_idx += 1;
    }
    mem
}

#[test]
fn abc_test() {
    let mut output = Vec::<u8>::new();
    exec_bf("++++[>++++[>++++<-]<-]>>+.+.+.".to_string(), Some(BufWriter::new(&mut output)));
    assert_eq!(output[0], b'A');
    assert_eq!(output[1], b'B');
    assert_eq!(output[2], b'C');
}