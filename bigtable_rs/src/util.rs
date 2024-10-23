pub fn get_end_key_for_prefix(start_key: &[u8]) -> Option<Vec<u8>> {
    let size = start_key.len();
    if size < 1 {
        return None;
    }

    let mut vector = vec![0; size];
    let mut carry = 1u8;

    for (i, key_part) in start_key.iter().enumerate().rev() {
        if *key_part != 0xFFu8 || carry == 0 {
            vector[i] = *key_part + carry;
            carry = 0;
        }
    }

    if carry == 1 {
        // overflow
        return None;
    }

    Some(vector)
}

#[cfg(test)]
mod tests {
    use super::get_end_key_for_prefix;
    #[test]
    fn get_end_key_works() {
        assert_eq!(get_end_key_for_prefix(&[]), None);
        assert_eq!(get_end_key_for_prefix(&[0x01u8]), Some([0x02u8].to_vec()));
        assert_eq!(
            get_end_key_for_prefix(&[0x01u8, 0xFFu8]),
            Some([0x02u8, 0x00u8].to_vec())
        );
        assert_eq!(
            get_end_key_for_prefix(&[0x21u8, 0xFFu8]),
            Some([0x22u8, 0x00u8].to_vec())
        );
        assert_eq!(
            get_end_key_for_prefix(&[0xFFu8, 0xF1u8, 0xFFu8]),
            Some([0xFFu8, 0xF2u8, 0x00u8].to_vec())
        );
        assert_eq!(get_end_key_for_prefix(&[0xFFu8]), None);
    }
}
