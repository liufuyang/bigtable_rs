fn get_end_key(start_key: &[u8]) -> Option<Vec<u8>> {
    // TODO
    Some(start_key.to_vec())
}

#[cfg(test)]
mod tests {
    use super::get_end_key;
    #[test]
    fn get_end_key_works() {
        assert_eq!(get_end_key(&[0x01u8]), Some([0x02u8].to_vec()));
        assert_eq!(
            get_end_key(&[0x01u8, 0xFFu8]),
            Some([0x02u8, 0x00u8].to_vec())
        );
        assert_eq!(get_end_key(&[0xFFu8]), None);
    }
}
