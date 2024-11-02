pub fn string_to_hash(s: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let hash_vec = match hex::decode(s) {
        Ok(hash) => hash,
        Err(_) => return Err("Invalid hash".into()),
    };

    let result: [u8; 32] = match hash_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => return Err("Invalid hash".into()),
    };

    Ok(result)
}