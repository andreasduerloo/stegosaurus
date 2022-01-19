use std::io;
use std::io::Read;
use std::fs;
use std::env;

enum Mode {
    Decode,
    Encode,
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

    for index in (*start_position as usize)..(*start_position as usize + text.len()*8) {
        let bitposition: usize = (index - *start_position as usize) % 8;
        let string_index = (index - *start_position as usize) / 8;
        // Build bitmask
        let bitmask: u8 = 1 << (7 - bitposition);
        // Find value
        let value = ( text[string_index] & bitmask ) >> (7 - bitposition);
        // Unset LSB and set it to the value
        *&mut image[index] = ( *&image[index] & 0xfe ) | value;

        // Increment
        *&mut out_index = index.try_into().unwrap();
    }

    out_index + 1
}

fn decode_string(image: &Vec<u8>, start_position: &u64) -> Vec<char> {
    let mut output_vec: Vec<char> = Vec::new();

    let mut current_char: u8 = 0;
    
    for index in (*start_position as usize)..image.len() {
        let bitposition: usize = (index - *start_position as usize) % 8;

        let value: u8 = (image[index] & 1) << 7 - bitposition;
        *&mut current_char |= value;

        if bitposition == 7 {
            let _ = &mut output_vec.push(current_char as char);
            *&mut current_char = 0;
        }
    
        // Check if the file contains the start flag
        if output_vec.len() == 7 {
            if output_vec[0..7] != ['`','S','T','A','R','T','`'] {
                panic!("This image does not contain a hidden message.");
            }
        }

        // Check if we've hit the end flag
        if output_vec.len() >= 12 {
            if output_vec[(output_vec.len() - 5)..(output_vec.len())] == ['`','E','N','D','`'] {
                return output_vec;
            }
        }
    }
    return output_vec;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut mode: Option<Mode> = None;

    if args.len() > 1 {
        if args[1] == "-d" || args[1] == "--decode" {
            *&mut mode = Some(Mode::Decode);
        } else if args[1] == "-e" || args[1] == "--encode" {
            *&mut mode = Some(Mode::Encode);
        }
    }

    if let Some(Mode::Decode) = mode {
        if args.len() != 3 {
            println!("Incorrect number of arguments. Usage: stegosaurus -d/--decode imagefile");
            return;
        }
        let image_path = &args[2];

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

        let message: Vec<char> = decode_string(&buffer, &start_address);

        for index in 7..(message.len()-5) {
            print!("{}", message[index]);
        }
    }

    if let Some(Mode::Encode) = mode {
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
    }

    if let None = mode {
        println!("Stegosaurus - steganography tool to write text to/read text from bitmap images.");
        print!("Usage:\nDecode: stegosaurus -d/--decode imagefile\nEncode: stegosaurus -e/--encode imagefile textfile destinationfile");
    }
}
