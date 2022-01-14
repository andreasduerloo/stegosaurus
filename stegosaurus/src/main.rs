use std::io;
use std::io::Read;
use std::fs;
use std::env;

enum Mode {
    Decode,
    Encode,
    Help,
}

fn decode_bytes(slice: &[u8]) -> u64 {
    let mut result: u64 = 0;

    for position in 0..slice.len() {
        result |= (slice[position] as u64) << 8*position;
    }

    result
}

// Returns the index of the NEXT (unwritten) byte after writing
fn encode_string(image: &mut Vec<u8>, text: &Vec<u8>, start_position: &u64) -> u64 {
    let mut out_index: u64 = *start_position;

    let mut power: u64 = 0;

    for index in (*start_position as usize)..(*start_position as usize + text.len()*8) {
        let string_index = (index - *start_position as usize) / 8;
        // Build bitmask
        let bitmask: u8 = 2_u8.pow((7 - &power % 8).try_into().unwrap());
        // Find value
        let value = ( text[string_index] & bitmask ) >> (7 - &power % 8);
        // Unset LSB
        *&mut image[index] &= 0xfe;
        // Set LSB to value
        *&mut image[index] |= value;

        // Increment
        *&mut power += 1;
        *&mut out_index = index.try_into().unwrap();
    }

    out_index + 1
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut mode: Mode = Mode::Help;

    if args.len() == 1 {
        *&mut mode = Mode::Help;
    } else if args[1] == "-d" || args[1] == "--decode" {
        *&mut mode = Mode::Decode;
    } else if args[1] == "-e" || args[1] == "--encode" {
        *&mut mode = Mode::Encode;
    } else {
        *&mut mode = Mode::Help;
    }

    match mode {
        Mode::Decode => {
            //
        },
        Mode::Encode => {
            if args.len() != 5 {
                println!("Incorrect number of arguments. Usage: stegosaurus -e/--encode imagefile textfile destinationfile");
                return;
            }

            let image_path = &args[2];
            let text_path = &args[3];
            let output_path = &args[4];

            // Image input
            let f = fs::File::open(image_path).expect("Could not read bitmap file");
            let mut reader = io::BufReader::new(f);
            let mut buffer: Vec<u8> = Vec::new();

            // Read file into vector
            reader.read_to_end(&mut buffer).unwrap();

            // Check if the file is a bitmap image
            if buffer[0..2] != [0x42, 0x4d] {
                println!("Error, the input file is not a bitmap.");
                return;
            }


            // Find the start of the pixel array
            let start_address: u64 = decode_bytes(&buffer[0x0a..0x0e]);

            // Print some info about the file. TO DO: make this a verbose flag
            // println!("File length in bytes: {}", decode_bytes(&buffer[2..6]));
            // println!("Image width in pixels: {}", decode_bytes(&buffer[0x12..0x16]));
            // println!("Image height in pixels: {}", decode_bytes(&buffer[0x16..0x1a]));

            // Text input
            let s = fs::read_to_string(text_path).expect("Could not read text file");
            let char_vec: Vec<u8> = s.bytes().collect();

            // println!("Input text is {} bytes", char_vec.len());

            let mut address: u64 = start_address;

            let start_flag = "`START`";
            let start_vec: Vec<u8> = start_flag.bytes().collect();

            let end_flag = "`END`";
            let end_vec: Vec<u8> = end_flag.bytes().collect();

            // Begin by putting in the start flag
            *&mut address = encode_string(&mut buffer, &start_vec, &address);

            // Now write the message
            *&mut address = encode_string(&mut buffer, &char_vec, &address);

            // Add the end flag
            let _ = encode_string(&mut buffer, &end_vec, &address);

            fs::write(output_path, buffer).unwrap();

        },
        Mode::Help => {
            println!("Stegosaurus - steganography tool to write text to/read text from bitmap images.");
            print!("Usage:\nDecode: stegosaurus -d/--decode imagefile\nEncode: stegosaurus -e/--encode imagefile textfile destinationfile");
        }
    }
}
