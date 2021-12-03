use core::panic;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    path::Path,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T,Error>;

fn build_ascii_font()-> Result<()> {
    let outDir = env::var("OUT_DIR").unwrap();

    let input_path = Path::new("asset/ascii_font.txt");
    let output_path = Path::new(&outDir).join("ascii_font.rs");

    println!("cargo:rerun-if-changed={}", input_path.display());

    let input = File::open(input_path)?;
    let input = BufReader::new(input);
    let output = File::create(output_path)?;
    let mut output = BufWriter::new(output);

    //writeln : arg1 にappend方式で書き込んでいく（\nをつけながら）
    writeln!(
        &mut output , 
        "pub(crate) const ASCII_FONT : [[u8;16]; 256]= ["
    )?;

    let mut last_index = None;
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();
        if line.is_empty(){
            continue;
        }

        //引数と引数の前の文字列を捨てる
        if let Some(rest) = line.strip_prefix("0x") {
            let (index_str, ch_str) = rest.split_at(2);
            let index = usize::from_str_radix(index_str, 16).unwrap();
            assert!(index == 0 || Some(index - 1) == last_index);
            last_index = Some(index);
            
            writeln!(&mut output, "    //0x{}{}", index_str,ch_str)?;
            writeln!(&mut output , "    [")?;
            for line in lines.by_ref() {
                let line = line?;
                let line = line.trim();
                if !line.starts_with(&['.', '@'][..]) {
                    break;
                }

                let mut output_num = 0;
                //chars で文字列をイテレートできる配列型に変換
                for ch in line.chars() {
                    let bit = match ch {
                        '.' => 0,
                        '@' => 1,
                        _ => panic!("invalid char: {:?}", ch),
                    };
                    output_num = (output_num << 1) | bit;
                }

                writeln!(&mut output, "        0b{:08b},", output_num)?;
            }
            writeln!(&mut output, "    ],")?;
        }
    }


    writeln!(&mut output,"];")?;

    Ok(())



}

fn main() -> Result<()> {
    build_ascii_font()?;
    Ok(())
}