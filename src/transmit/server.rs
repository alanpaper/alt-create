use anyhow::{Context, Result};
use std::{path::Path, time::Instant};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .context("Failed to bind to port 8080")?;
    println!("Server running on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let start_time = Instant::now();

            // Receive filename
            let mut filename_buf = [0u8; 256];
            let n = socket
                .read(&mut filename_buf)
                .await
                .context("Failed to read filename")?;
            let filename = String::from_utf8_lossy(&filename_buf[..n])
                .trim_end_matches('\0')
                .to_string();
            let filepath = Path::new(&filename)
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

            // Receive file size
            let mut size_buf = [0u8; 8];
            socket
                .read_exact(&mut size_buf)
                .await
                .context("Failed to read file size")?;
            let file_size = u64::from_le_bytes(size_buf);
            println!("Receiving file: {} ({} bytes)", filepath, file_size);

            // Create file
            let mut file = tokio::fs::File::create(filepath)
                .await
                .context("Failed to create file")?;
            let mut received_size = 0;

            // 在 server.rs 的循环中添加连接保持检查
            while received_size < file_size {
                // 读取分块大小
                let mut chunk_size_buf = [0u8; 4];
                if let Err(e) = socket.read_exact(&mut chunk_size_buf).await {
                    eprintln!("Failed to read chunk size: {}", e);
                    break; // 退出循环但保持连接
                }
                let chunk_size = u32::from_le_bytes(chunk_size_buf) as usize;

                // 读取分块数据
                let mut chunk = vec![0u8; chunk_size];
                if let Err(e) = socket.read_exact(&mut chunk).await {
                    eprintln!("Failed to read chunk data: {}", e);
                    break;
                }

                // 读取校验和
                let mut checksum = [0u8; 1];
                if let Err(e) = socket.read_exact(&mut checksum).await {
                    eprintln!("Failed to read checksum: {}", e);
                    break;
                }

                // 验证校验和
                let expected_checksum = calculate_checksum(&chunk);
                if checksum[0] == expected_checksum {
                    socket.write_all(&[1]).await?; // 发送 ACK
                    file.write_all(&chunk).await?;
                    received_size += chunk_size as u64;
                } else {
                    socket.write_all(&[0]).await?; // 发送 NAK
                }
            }
            let elapsed = start_time.elapsed();
            println!(
                "File received: {} ({} bytes) in {:.2?}",
                filepath, received_size, elapsed
            );
            Ok::<(), anyhow::Error>(())
        });
    }
}

fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
}
