use futures::stream::StreamExt;
use async_std::fs::File;
use futures::AsyncWriteExt;
use std::{fs::File as SyncFile, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.sec.gov/Archives/edgar/full-index/1994/QTR1/company.idx";
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let mut response_body = response.bytes_stream();
    let mut async_file = File::create("company.async.idx").await?;
    let mut sync_file = SyncFile::create("company.sync.idx")?;
    while let Some(buffer) = response_body.next().await {
        let buffer = buffer?;
        let _ = async_file.write(&buffer).await?;
        async_file.sync_all().await?;
        sync_file.write(&buffer)?;
    }
    return Ok(());
}
