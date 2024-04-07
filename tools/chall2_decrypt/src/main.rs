use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    InvalidChecksum,
    InvalidCharacter(u8),
}

fn wBreedMon2Exp(start: u8, data: &[u8; 0x200]) -> u8 {
    let mut i = 0x78;
    let mut idx = start as usize;
    let mut res = 0u8;

    loop {
        res = res.wrapping_add(data[idx]);
        res = (res & 0x0F) << 4 | (res & 0xF0) >> 4;
        idx += 1;

        res ^= data[idx];
        res = (res & 0x0F) << 4 | (res & 0xF0) >> 4;
        idx += 1;

        i -= 1;
        if i == 0 {
            break;
        }
    }

    res
}

fn decrypt_password(data: &[u8; 0x200]) -> Result<[u8; 10], Error> {
    let mut password: [u8; 10] = [0; 10];
    let checksum = wBreedMon2Exp(0, &data);

    if checksum != 0x35 {
        return Err(Error::InvalidChecksum);
    }

    // println!("Checksum: {:#02X}", checksum);

    for start in (1..=10).rev() {
        let mut res = wBreedMon2Exp(start, &data);

        res &= 0x1F;
        res = res.wrapping_add(0x84).wrapping_add(start);
        password[(start as usize) - 1] = res;
    }

    password.reverse();
    Ok(password)
}

fn decode_text(text: &[u8]) -> Result<String, Error> {
    let mut char_map: HashMap<u8, char> = HashMap::new();
    char_map.insert(0x80, 'A');
    char_map.insert(0x81, 'B');
    char_map.insert(0x82, 'C');
    char_map.insert(0x83, 'D');
    char_map.insert(0x84, 'E');
    char_map.insert(0x85, 'F');
    char_map.insert(0x86, 'G');
    char_map.insert(0x87, 'H');
    char_map.insert(0x88, 'I');
    char_map.insert(0x89, 'J');
    char_map.insert(0x8a, 'K');
    char_map.insert(0x8b, 'L');
    char_map.insert(0x8c, 'M');
    char_map.insert(0x8d, 'N');
    char_map.insert(0x8e, 'O');
    char_map.insert(0x8f, 'P');
    char_map.insert(0x90, 'Q');
    char_map.insert(0x91, 'R');
    char_map.insert(0x92, 'S');
    char_map.insert(0x93, 'T');
    char_map.insert(0x94, 'U');
    char_map.insert(0x95, 'V');
    char_map.insert(0x96, 'W');
    char_map.insert(0x97, 'X');
    char_map.insert(0x98, 'Y');
    char_map.insert(0x99, 'Z');

    char_map.insert(0x9a, '(');
    char_map.insert(0x9b, ')');
    char_map.insert(0x9c, ':');
    char_map.insert(0x9d, ';');
    char_map.insert(0x9e, '[');
    char_map.insert(0x9f, ']');

    char_map.insert(0xa0, 'a');
    char_map.insert(0xa1, 'b');
    char_map.insert(0xa2, 'c');
    char_map.insert(0xa3, 'd');
    char_map.insert(0xa4, 'e');
    char_map.insert(0xa5, 'f');
    char_map.insert(0xa6, 'g');
    char_map.insert(0xa7, 'h');
    char_map.insert(0xa8, 'i');
    char_map.insert(0xa9, 'j');
    char_map.insert(0xaa, 'k');
    char_map.insert(0xab, 'l');
    char_map.insert(0xac, 'm');
    char_map.insert(0xad, 'n');
    char_map.insert(0xae, 'o');
    char_map.insert(0xaf, 'p');
    char_map.insert(0xb0, 'q');
    char_map.insert(0xb1, 'r');
    char_map.insert(0xb2, 's');
    char_map.insert(0xb3, 't');
    char_map.insert(0xb4, 'u');
    char_map.insert(0xb5, 'v');
    char_map.insert(0xb6, 'w');
    char_map.insert(0xb7, 'x');
    char_map.insert(0xb8, 'y');
    char_map.insert(0xb9, 'z');

    let mut res = String::new();
    for b in text {
        let c = char_map.get(b).ok_or(Error::InvalidCharacter(*b))?;
        res.push(*c);
    }

    Ok(res)
}

fn generate_data() -> [u8; 0x200] {
    // Generate a random array
    let mut data: [u8; 0x200] = [0; 0x200];
    for b in data.iter_mut() {
        *b = fastrand::u8(..);
    }

    // Set some of the constraints
    data[0xF0] = 0x79;
    data[0x0B] = 0x05;
    data[0x7F] = 0x23;
    data[0x2B] = 0x02;
    data[0x2C] = 0x04;
    data[0xE8] = 0x01;

    data
}

fn main() {
    loop {
        let data = generate_data();
        let password = decrypt_password(&data);

        if let Ok(password) = password {
            println!("Password: {:?}", decode_text(&password));

            // for b in data {
            //     print!("{:02X} ", b);
            // }
            //
            // println!();
            // break;
        }
    }
}
