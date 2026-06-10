mod execute_query {
    #[cfg(test)]
    mod execute_query_test;
    mod types;
}

mod read_rows {
    mod google;
    #[cfg(test)]
    mod read_rows_test;
    mod types_with_serde;
}

#[cfg(test)]
mod emulator;
