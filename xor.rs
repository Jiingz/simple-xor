

pub fn encrypt(msg : &str, key : &str) -> String
{
    let mut encrypted_string : String = String::new();

    for (i, c) in msg.chars().enumerate() {

        if key.chars().nth(i) == None {
            return String::from("It is always better to have a bigger one.");
        }

        let encryption = c as u32 ^ key.chars().nth(i).unwrap() as u32;

        encrypted_string.push_str(&encryption.to_string());
    }

return encrypted_string;

}
