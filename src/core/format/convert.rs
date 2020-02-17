
#[allow(dead_code)]
fn u8_to_string(input: &Vec<u8>) -> Result<String, String> {
    let ret = hex::encode(input);
    decode_hex(ret.as_str())
}

pub fn decode_hex(input: &str) -> Result<String, String> {
    let ret = hex::decode(input).map_err(|e| e.to_string())?;
    Ok(String::from_utf8(ret).expect("fail convert utf8").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_hex() {
        assert_eq!("BARCODE", decode_hex("424152434F4445").unwrap().as_str());
        assert_eq!(true, decode_hex("424152434F444Z").is_err());
    }

    #[test]
    fn test_u8_to_string() {
        let data: Vec<u8> = vec![66, 65, 82, 67, 79, 68, 69];
        assert_eq!("BARCODE", u8_to_string(&data).unwrap().as_str());

        let data: Vec<u8> = vec![66, 65, 82, 67, 79, 68, 69];
        assert_eq!("BARCODE", u8_to_string(&data).unwrap().as_str());
    }
}
