use crate::bigtable::{Error, Result, RowCell, RowKey};
use crate::google::bigtable::v2::read_rows_response::cell_chunk::RowStatus;
use crate::google::bigtable::v2::read_rows_response::CellChunk;
use crate::google::bigtable::v2::ReadRowsResponse;
use log::trace;
use std::time::{Duration, Instant};
use tonic::Streaming;

/// As each `CellChunk` could be only part of a cell, this method reorganize multiple `CellChunk`
/// from multiple `ReadRowsResponse` into a `Vec<(RowKey, Vec<RowCell>)>`.
pub async fn decode_read_rows_response(
    timeout: &Option<Duration>,
    mut rrr: Streaming<ReadRowsResponse>,
) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
    let mut rows: Vec<(RowKey, Vec<RowCell>)> = vec![];

    let started = Instant::now();
    while let Some(res) = rrr.message().await? {
        if let Some(timeout) = timeout.as_ref() {
            if Instant::now().duration_since(started) > *timeout {
                return Err(Error::TimeoutError(timeout.as_secs()));
            }
        }
        let rows_part = decode_read_rows_response_to_vec(res.chunks);
        for part in rows_part.into_iter() {
            match part {
                Ok(part) => rows.push(part),
                Err(e) => return Err(e),
            }
        }
    }
    Ok(rows)
}

pub fn decode_read_rows_response_to_vec(
    chunks: Vec<CellChunk>,
) -> Vec<Result<(RowKey, Vec<RowCell>)>> {
    let mut rows: Vec<Result<(RowKey, Vec<RowCell>)>> = vec![];
    let mut row_key = None;
    let mut row_data: Vec<RowCell> = vec![];

    let mut cell_family_name = None;
    let mut cell_name = None;
    let mut cell_timestamp = 0;
    let mut cell_value = vec![];
    let mut cell_labels = vec![];

    let mut start_new_cell = false;
    let mut committed_row_cell_count = 0usize;
    let mut start_new_row = false; // Marker for starting a new row. A commit will set this as false

    // TODO: Moving tests from copy.json to temp.json and update the logic below to let tests all pass in the end...

    if chunks.is_empty() {
        return rows;
    }

    for (i, mut chunk) in chunks.into_iter().enumerate() {
        // The comments for `read_rows_response::CellChunk` provide essential details for
        // understanding how the below decoding works...
        trace!("chunk {}: {:?}", i, chunk.value);

        // Starting a new row?
        if !chunk.row_key.is_empty() {
            if row_key.is_none() || row_key.take().unwrap() != chunk.row_key {
                // a new key comes, start_new_row should be false at this time
                if start_new_row {
                    rows.truncate(committed_row_cell_count);
                    rows.push(Err(Error::ChunkError(
                        "Invalid - no commit before key changes".to_owned(),
                    )));
                    return rows;
                }
                start_new_row = true;
            }
            row_key = Some(chunk.row_key);
        }

        // when starting a new cell with new family name, then a qualifier must exist
        if chunk.family_name.is_some()
            && !chunk.family_name.eq(&cell_family_name)
            && chunk.qualifier.is_none()
        {
            rows.truncate(committed_row_cell_count);
            rows.push(Err(Error::ChunkError(
                "new col family but no specified qualifier".to_owned(),
            )));
            return rows;
        }

        // start a new cell with the existing cell_name or new cell_name (chunk.qualifier)
        if (start_new_cell && cell_name.is_some()) || chunk.qualifier.is_some() {
            cell_value = Vec::with_capacity(chunk.value_size as usize);
            // when a new cell with the same qualifier starts, we need to reuse the old cell_name and cell_family_name
            cell_family_name = chunk.family_name.or(cell_family_name);
            cell_name = chunk.qualifier.or(cell_name);
            cell_timestamp = chunk.timestamp_micros;
            cell_labels = chunk.labels;
            start_new_cell = false;
        }

        cell_value.append(&mut chunk.value);
        // last chunk for the cell?
        if chunk.value_size == 0 {
            // Close up the cell
            if cell_name.is_some() {
                let row_cell = RowCell {
                    family_name: cell_family_name.clone().unwrap_or("".to_owned()),
                    qualifier: cell_name.clone().unwrap(), // checked above
                    value: cell_value,
                    timestamp_micros: cell_timestamp,
                    labels: cell_labels,
                };
                cell_value = vec![]; // borrow checker
                cell_labels = vec![];
                row_data.push(row_cell);
            }
            // make sure we start a new cell in case the qualifier doesn't change
            start_new_cell = true;
        }

        // End of a row?
        match chunk.row_status {
            None => {
                // more for this row, don't push to row_data or rows vector, let the next
                // chunk close up those vectors.
            }
            Some(RowStatus::CommitRow(flag)) => {
                if let Some(row_key) = row_key.clone() {
                    rows.push(Ok((row_key, row_data)));
                    row_data = vec![];
                }
                if flag {
                    committed_row_cell_count = rows.len();
                    start_new_row = false;
                }
            }
            Some(RowStatus::ResetRow(_)) => {
                // ResetRow indicates that the client should drop all previous chunks for
                // `row_key`, as it will be re-read from the beginning.
                row_key = None;
                row_data = vec![];
            }
        }
    }

    if committed_row_cell_count == 0 {
        return vec![Err(Error::ChunkError("No rows committed".to_owned()))];
    }

    if start_new_row {
        rows.truncate(committed_row_cell_count);
        rows.push(Err(Error::ChunkError(
            "Invalid - last row missing commit".to_owned(),
        )));
        return rows;
    }

    return rows;
}
