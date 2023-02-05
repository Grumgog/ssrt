mod srt_block;
mod tests;
mod timestamp;

use clap::Parser;
use std::{fs, io::Write};

use crate::srt_block::SRTBlock;

/// Program to shift timestamps in srt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file for data.
    #[arg(short, long)]
    input_file: String,
    /// Output file for data.
    #[arg(short, long)]
    output_file: String,
    /// shift value in ms.
    #[arg(short, long)]
    shift: i64,
}

fn main() {
    let args = Args::parse();

    let file_content = fs::read_to_string(args.input_file)
        .expect("Программа должна иметь возможность прочитать файл");

    let mut output_file = fs::File::create(args.output_file)
        .expect("Программа должна иметь возможность открыть файл для записи");

    let file_raw_block = file_content.split("\r\n\r\n");

    for block in file_raw_block {
        if block.len() != 0 {
            let mut r = SRTBlock::from_str(block);
            r.shift_timestamps(args.shift);
            output_file
                .write(format!("{}\r\n", r.to_str()).to_string().as_bytes())
                .expect("Не возможно записать выходные данные");
        }
    }
}
