fn get_end_key(start_key: &[u8]) -> Option<Vec<u8>> {
    let size = start_key.len();
    if size < 1 {
        return None;
    }

    let mut vector = Vec::with_capacity(size);

    let mut i = size - 1;

    let mut carry: u8 = 1;

    loop {
        if start_key[i] == 0xFFu8 && carry == 1 {
            vector.push(0);
        } else {
            vector.push(start_key[i] + carry);
            carry = 0;
        }

        match i.checked_sub(1) {
            Some(_i) => i = _i,
            None => break,
        }
    }

    if i == 0 && carry == 1 {
        // overflow
        return None;
    }

    vector.reverse();

    Some(vector)
}

#[cfg(test)]
mod tests {
    use super::get_end_key;
    #[test]
    fn get_end_key_works() {
        assert_eq!(get_end_key(&[]), None);
        assert_eq!(get_end_key(&[0x01u8]), Some([0x02u8].to_vec()));
        assert_eq!(
            get_end_key(&[0x01u8, 0xFFu8]),
            Some([0x02u8, 0x00u8].to_vec())
        );
        assert_eq!(get_end_key(&[0xFFu8]), None);
    }
}
