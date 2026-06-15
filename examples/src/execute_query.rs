/// Demonstrates execute_query_stream with checkpoint-based retry.
///
/// On a transient stream error the query resumes from the last resume token
/// issued by the server, avoiding re-reading rows that were already received.
/// If no checkpoint has been issued yet the query restarts from the beginning.
///
/// Run against the emulator:
///   gcloud beta emulators bigtable start --host-port=localhost:8086
///   export BIGTABLE_EMULATOR_HOST=localhost:8086
///   cargo run --bin execute_query
use std::error::Error;
use std::time::Duration;

use bigtable_rs::bigtable::{self, ExecuteQueryRetryContext};
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::ExecuteQueryRequest;

const MAX_RETRIES: usize = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let project_id = "project-1";
    let instance_id = "instance-1";
    let table_name = "table-1";
    let channel_size = 4;
    let timeout = Duration::from_secs(10);

    let connection = bigtable::BigTableConnection::new(
        project_id,
        instance_id,
        true,
        channel_size,
        Some(timeout),
    )
    .await?;
    let mut bigtable = connection.client();

    let instance_name = format!("projects/{project_id}/instances/{instance_id}");
    let sql = format!("SELECT * FROM {table_name}");

    // retry_ctx is None for the initial call and Some on each retry.
    // It carries the prepared_query bytes (so re-prepare is skipped) and the
    // last resume_token (so already-delivered rows are not re-sent).
    let mut retry_ctx: Option<ExecuteQueryRetryContext> = None;

    for attempt in 0..=MAX_RETRIES {
        if attempt > 0 {
            println!("Retry attempt {attempt}/{MAX_RETRIES} from last checkpoint...");
        }

        // Fresh query: set `query`. Retry: leave query empty; retry_ctx supplies
        // the prepared plan and resume token.
        #[allow(deprecated)]
        let request = ExecuteQueryRequest {
            instance_name: instance_name.clone(),
            query: if retry_ctx.is_none() {
                sql.clone()
            } else {
                String::new()
            },
            ..ExecuteQueryRequest::default()
        };

        // Clone so retry_ctx is still available if the RPC itself fails before
        // the stream starts.
        let (metadata, mut stream) = match bigtable
            .execute_query_stream(request, retry_ctx.clone())
            .await
        {
            Ok(r) => r,
            Err(e) => {
                eprintln!("RPC error on attempt {attempt}: {e}");
                if attempt < MAX_RETRIES {
                    continue;
                }
                return Err(e.into());
            }
        };

        if attempt == 0 {
            println!("Schema: {:?}", metadata.schema);
        }

        let mut stream_error = false;
        loop {
            match stream.next().await {
                None => break, // stream finished cleanly
                Some(Ok(batch)) => {
                    for row in &batch.rows {
                        println!("row: {} column(s)", row.0.len());
                    }
                    // Save the latest checkpoint so a subsequent retry can
                    // resume from here rather than from the beginning.
                    if let Some(c) = batch.retry_context {
                        retry_ctx = Some(c);
                    }
                }
                Some(Err(e)) => {
                    eprintln!("Stream error on attempt {attempt}: {e}");
                    stream_error = true;
                    break;
                }
            }
        }

        if !stream_error {
            break; // success — exit the retry loop
        }
        if attempt == MAX_RETRIES {
            return Err(format!("query failed after {MAX_RETRIES} retries").into());
        }
    }

    Ok(())
}
