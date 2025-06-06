use anyhow::{Context, Result};
use std::{path::Path, time::Instant};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::{timeout, Duration},
};

const MAX_FILENAME_LEN: usize = 4096; // 4KB最大文件名长度

pub async fn server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .context("Failed to bind to port 8080")?;
    println!("Server running on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let start_time = Instant::now();

            // 接收文件名长度
            let mut name_len_buf = [0u8; 2];
            timeout(Duration::from_secs(5), socket.read_exact(&mut name_len_buf))
                .await?
                .context("Failed to read filename length")?;
            let name_len = u16::from_le_bytes(name_len_buf) as usize;

            // 检查文件名长度是否合法
            if name_len > MAX_FILENAME_LEN {
                return Err(anyhow::anyhow!("Filename too long"));
            }

            // 接收文件名
            let mut filename_buf = vec![0u8; name_len];
            timeout(Duration::from_secs(5), socket.read_exact(&mut filename_buf))
                .await?
                .context("Failed to read filename")?;
            let filename = String::from_utf8(filename_buf)?;

            // 安全提取文件名
            let filename = Path::new(&filename)
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

            // 防止路径穿越攻击
            if filename.contains("..") || filename.starts_with('/') || filename.starts_with('\\') {
                return Err(anyhow::anyhow!("Invalid filename format"));
            }

            // 接收文件大小
            let mut size_buf = [0u8; 8];
            timeout(Duration::from_secs(5), socket.read_exact(&mut size_buf))
                .await?
                .context("Failed to read file size")?;
            let file_size = u64::from_le_bytes(size_buf);
            println!("Receiving file: {} ({} bytes)", filename, file_size);

            // 创建文件
            let mut file = tokio::fs::File::create(filename)
                .await
                .context("Failed to create file")?;
            let mut received_size = 0;

            while received_size < file_size {
                // 读取分块大小
                let mut chunk_size_buf = [0u8; 4];
                if let Err(e) = timeout(
                    Duration::from_secs(30),
                    socket.read_exact(&mut chunk_size_buf),
                )
                .await
                {
                    eprintln!("Failed to read chunk size: {}", e);
                    break;
                }
                let chunk_size = u32::from_le_bytes(chunk_size_buf) as usize;

                // 读取分块数据
                let mut chunk = vec![0u8; chunk_size];
                if let Err(e) = timeout(
                    Duration::from_secs(30),
                    socket.read_exact(&mut chunk),
                )
                .await
                {
                    eprintln!("Failed to read chunk data: {}", e);
                    break;
                }

                // 读取校验和
                let mut checksum = [0u8; 1];
                if let Err(e) = timeout(
                    Duration::from_secs(5),
                    socket.read_exact(&mut checksum),
                )
                .await
                {
                    eprintln!("Failed to read checksum: {}", e);
                    break;
                }

                // 验证校验和
                let expected_checksum = calculate_checksum(&chunk);
                if checksum[0] == expected_checksum {
                    timeout(Duration::from_secs(5), socket.write_all(&[1]))
                        .await?
                        .context("Failed to send ACK")?;
                    timeout(Duration::from_secs(5), file.write_all(&chunk))
                        .await?
                        .context("Failed to write file chunk")?;
                    received_size += chunk_size as u64;
                    
                    // 打印进度
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 {
                        (received_size as f64) / (1024.0 * 1024.0) / elapsed
                    } else {
                        0.0
                    };
                    println!(
                        "Progress: {:.2}% - Speed: {:.2} MB/s",
                        (received_size as f64 / file_size as f64) * 100.0,
                        speed
                    );
                } else {
                    eprintln!("Checksum mismatch for chunk");
                    timeout(Duration::from_secs(5), socket.write_all(&[0]))
                        .await?
                        .context("Failed to send NAK")?;
                }
            }
            
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let avg_speed = (received_size as f64) / (1024.0 * 1024.0) / elapsed;
                println!(
                    "File received: {} ({} bytes) in {:.2?} - Avg speed: {:.2} MB/s",
                    filename, received_size, elapsed, avg_speed
                );
            } else {
                println!(
                    "File received: {} ({} bytes)",
                    filename, received_size
                );
            }
            
            Ok::<(), anyhow::Error>(())
        });
    }
}

fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
}