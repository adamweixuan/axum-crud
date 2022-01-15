use anyhow::Ok;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::debug;

#[allow(dead_code)]
async fn create_file(path: impl AsRef<Path>) -> Result<bool, anyhow::Error> {
    let mut file = File::create(path).await?;
    file.write_all(b"hello, world!").await?;
    Ok(true)
}

#[allow(dead_code)]
async fn open_file(path: impl AsRef<Path>) -> Result<bool, anyhow::Error> {
    let file = File::open(path).await?;
    debug!("file info {:?}", file);
    Ok(true)
}

#[allow(dead_code)]
async fn read_from_file(path: impl AsRef<Path>) -> Result<Vec<u8>, anyhow::Error> {
    let mut file = File::open(path).await?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

// async fn write_to_file(path: impl AsRef<Path>) -> Result<usize, anyhow::Error> {
//     let mut file = File::open(path).await?;
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        create_file("./test_create.txt").await.unwrap();
    }

    #[tokio::test]
    async fn test_open() {
        open_file("./test_create.txt").await.unwrap();
        match open_file("./test_create1.txt").await {
            Result::Ok(_) => {
                println!("ok...");
            }
            Err(err) => {
                println!("error ... {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_read() {
        println!("current exe {:?}", std::env::current_exe());
        println!("current path {:?}", std::env::current_dir());
        let res = read_from_file(".env").await;

        res.map(|val| {
            println!("read file success : {:?}", String::from_utf8(val));
        })
        .map_err(|err| {
            println!("read file error {:?}", &err);
        });
    }
}
