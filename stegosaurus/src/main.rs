use std::io;
use std::io::Read;
use std::fs;

enum Mode { // TO DO make this a command line parameter
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
        *&mut image[index] |= (0x0 | value);

        // Increment
        *&mut power += 1;
        *&mut out_index = index.try_into().unwrap();
    }

    out_index + 1
}

fn main() {

    // let mut verbose: bool = false;
    // let mut mode: Mode = Mode::Decode;

    // TO DO - make these into command line arguments
    let image_path = "C:/users/andre/Desktop/ciske.bmp";
    let text_path = "C:/users/andre/Desktop/input_text.txt";
    let output_path = "C:/users/andre/Desktop/outimage.bmp";

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
    println!("File length in bytes: {}", decode_bytes(&buffer[2..6]));
    println!("Image width in pixels: {}", decode_bytes(&buffer[0x12..0x16]));
    println!("Image height in pixels: {}", decode_bytes(&buffer[0x16..0x1a]));

    // Text input
    let s = fs::read_to_string(text_path).expect("Could not read text file");
    let char_vec: Vec<u8> = s.bytes().collect();

    println!("Input text is {} bytes", char_vec.len());

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
