use rand::Rng;

fn main() {
    let key: String = "this is a secret key".to_string();
    let nonce = rand::thread_rng().gen_range(0..255); //this is a random value that is used
    let padded_key = padding(key);
    println!("{}", padded_key);
    let counter: u8 = 0;
    // x, y, z, w are the four words of salsa20
    // x: The first 64 bits (8 bytes) of the key.
    // y: The next 64 bits (8 bytes) of the key.
    // z: The nonce (64 bits, 8 bytes).
    // w: The counter (64 bits, 8 bytes).
    let x = &padded_key[0..8];
    let y = &padded_key[8..16];
    let z = nonce;
    let w = counter;
}

//ensuring that the key is of 32 bytes/256 bits
fn padding(mut string: String) -> String {
    let mut is_padded = false;
    while is_padded == false {
        let length = string.len();
        if length > 32 {
            panic!("[[[[[[[[KEY TO LONG, key must be <= than 32 bytes]]]]]]]]");
        } else if length < 32 {
            string.push_str("X"); //chosen this char for easy debugging and clear. could change later
        } else {
            is_padded = true;
        }
    }

    string
}

