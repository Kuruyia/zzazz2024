mod gb;

#[macro_use]
extern crate afl;

use std::{fs, io, thread};
use std::path::PathBuf;
use clap::Parser;
use md5::Digest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_file_path: PathBuf,
}

#[derive(Debug)]
enum Error {
    FileRead(io::Error),
}

const KNOWN_DIGESTS: [Digest; 4] = [
    // 77152813e184f4af1fee56d86a6dc9a8
    Digest([
        0x77, 0x15, 0x28, 0x13, 0xe1, 0x84, 0xf4, 0xaf,
        0x1f, 0xee, 0x56, 0xd8, 0x6a, 0x6d, 0xc9, 0xa8,
    ]),
    // fefc5bb0d94f1bd6d572e99d689b96eb
    Digest([
        0xfe, 0xfc, 0x5b, 0xb0, 0xd9, 0x4f, 0x1b, 0xd6,
        0xd5, 0x72, 0xe9, 0x9d, 0x68, 0x9b, 0x96, 0xeb,
    ]),
    // 21ac85860c8dd06557c9e68f593eef19
    Digest([
        0x21, 0xac, 0x85, 0x86, 0x0c, 0x8d, 0xd0, 0x65,
        0x57, 0xc9, 0xe6, 0x8f, 0x59, 0x3e, 0xef, 0x19,
    ]),
    // 54ffe78224c95daad006b297240b2f02
    Digest([
        0x54, 0xff, 0xe7, 0x82, 0x24, 0xc9, 0x5d, 0xaa,
        0xd0, 0x06, 0xb2, 0x97, 0x24, 0x0b, 0x2f, 0x02,
    ]),
];

fn step(gb: &mut gb::Gameboy, request: &[u8]) -> Option<String> {
    gb.step();

    if gb.registers.pc == 0x0168 {
        // ROM is waiting for us, write the request
        // println!("ROM is waiting for us, writing request...");
        gb.write_bytes(0xC000, request);
        gb.write_byte(0xFF80, 0x02);
    } else if gb.registers.pc == 0x07D0 {
        // ROM is done, read the response
        // println!("ROM is done, reading response...");

        let response = gb.read_bytes(0xC800, 0x800);
        let response_cleaned = response.iter().take_while(|&&b| b != 0).cloned().collect::<Vec<u8>>();

        return Some(String::from_utf8_lossy(&response_cleaned).to_string());
    }

    None
}

fn task(rom_contents: Vec<u8>) {
    println!("Task executes on thread: {:?}", thread::current().id());

    // Create a new Gameboy instance
    let mut gb = gb::Gameboy::new();
    gb.load_rom(rom_contents);

    let mut request = " /secret xxxx".as_bytes().to_vec();
    let request_len = request.len();

    #[cfg(not(fuzzing))]
    loop {
        let current = fastrand::u32(..);
        gb.reset();

        // Set the last four bytes of the request to the current value of the loop counter
        request[request_len - 4] = (current & 0xFF) as u8;
        request[request_len - 3] = ((current >> 8) & 0xFF) as u8;
        request[request_len - 2] = ((current >> 16) & 0xFF) as u8;
        request[request_len - 1] = ((current >> 24) & 0xFF) as u8;

        loop {
            if let Some(response) = step(&mut gb, &request) {
                // println!("Response: {}", response);
                // println!("Digest: {:x}", md5::compute(response.as_bytes()));

                let oops = &response[131..=135];
                if oops != "Oops!" {
                    println!("OOPS! Found it!");
                    println!("Request: {:?}", request);
                    println!("Response: {}", response);
                    println!("Current: {}", current);
                }

                break;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    // Get the path to the save file from the CLI arguments
    let args = Args::parse();
    println!("Reading ROM file from {:?}", args.rom_file_path);

    // Read the save file into a byte vector
    let rom_contents = fs::read(args.rom_file_path).map_err(Error::FileRead)?;

    let pool = rayon::ThreadPoolBuilder::new().num_threads(24).build().unwrap();

    for i in 0..23 {
        let rom_contents = rom_contents.clone();

        pool.spawn(move || {
            task(rom_contents);
        });
    }

    task(rom_contents);

    #[cfg(fuzzing)]
    fuzz!(|data: &[u8]| {
        loop {
            if let Some(response) = step(&mut gb, request) {
                let digest = md5::compute(response.as_bytes());
                // println!("Digest: {:x}", digest);
                // fs::write("fuzzing_output", &data).unwrap();

                if !KNOWN_DIGESTS.iter().any(|&d| d == digest) {
                    panic!("Unexpected digest: {:x}", digest);
                }

                break;
            }
        }
    });

    Ok(())
}