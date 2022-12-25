fn main() {
    println!("######################### ");
    println!("AND operator");
    let bin1: u8 = 0b01101011;
    let bin1_in_binary = parser_tobinary(bin1);
    //operator and
    // how can i get just last 3 bits?

    let mask_for_last3: u8 = 0b00000111;
    let last3: u8 = bin1 & mask_for_last3;
    let last3_in_binary = parser_tobinary(last3);

    println!("binary => {}", bin1_in_binary);
    println!("binary of the last 3 bits => {}", last3_in_binary);
    println!("\nshift operator\n");

    let mask_for_5_4: u8 = 0b00011000;
    let bit_5_4: u8 = bin1 & mask_for_5_4;

    // shift >> divides by 2
    // shift << multiplys by 2
    let bit_5_4_shifted: u8 = bit_5_4 >> 3; // 00000011
    let bit_5_4_shifted_binary = parser_tobinary(bit_5_4_shifted);

    print!("binary of 4 an 5 bits => {}", bit_5_4_shifted_binary);
    // AND operator extract information from binary

    //#################################################
    println!("\n######################### \n");
    println!("OR operator");

    print!("how to set a bit in arbitrary position in null byte");

    let mut null_bit: u8 = 0b00000000;

    let bin2: u8 = 0b00000101;
    let bin2_in_binary = parser_tobinary(bin2);

    let bin2_shifted_by_2: u8 = bin2 << 2; // 00010100
    let null_bit_in_binary_before = parser_tobinary(null_bit);
    println!(
        "binary before intersection => \n1) {}\n2) {}",
        null_bit_in_binary_before, bin2_in_binary
    );
    null_bit |= bin2_shifted_by_2; // null_bit = 00010100
    let null_bit_in_binary = parser_tobinary(null_bit);
    println!("binary of a set second  byte in some position of a null byte -> {}", null_bit_in_binary);
    //#################################################
    println!("\n######################### \n");
    println!("Oparations");

    println!("\nClear a bit");
    // invert bits
    let bin3: u8 = 0b10111011;
    let inverted_bin3 = !bin3; //01000100
    let bin3_in_binary = parser_tobinary(inverted_bin3);
    // let inverted_bin3_in_binary = parser_tobinary(inverted_bin3); // 1000100

    println!("binary => {}", bin3_in_binary);
    // println!("inverted binary => {}", inverted_bin3_in_binary);

    //how to clear a bit ?
    let mask_for_6_bit: u8 = 0b00100000;
    let mask_for_6_bit_inverted = !mask_for_6_bit;

    let bin3_6_bit_cleared: u8 = bin3 & mask_for_6_bit_inverted;
    let bin3_6_bit_cleared_in_binary = parser_tobinary(bin3_6_bit_cleared);

    println!("6 bit cleared => {}", bin3_6_bit_cleared_in_binary);
    //###################################################
    println!("\n######################### \n");
    println!("XOR operator");

    let first_op: u8 = 0b10100110;

    let first_op_binary = parser_tobinary(first_op);
    let second_op: u8 = 0b11000101;

    let second_op_binary = parser_tobinary(second_op);
    print!("first Byte {}    ", first_op_binary);
    print!("second Byte {}   ", second_op_binary);

    let result = first_op ^ second_op;
    let result_in_binary = parser_tobinary(result);
    println!("Result of XOR operation: {}  ", result_in_binary);
}

fn parser_tobinary(number: u8) -> String {
    format!("{number:b}")
}
