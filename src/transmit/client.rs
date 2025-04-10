use anyhow::Result;
use std::path::PathBuf;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration, Instant},
};

const MAX_RETRIES: u32 = 3;

pub async fn client(file_path: PathBuf, server_ip: &str) -> Result<()> {
    println!("Connecting to server at {}...", server_ip);
    let mut file = tokio::fs::File::open(&file_path).await?;
    let filename = file_path.file_name().unwrap().to_str().unwrap();

    let mut socket = TcpStream::connect(format!("{}:8080", server_ip)).await?;
    println!("Sending file: {} to {}", filename, server_ip);

    // 发送文件名和文件大小
    socket.write_all(filename.as_bytes()).await?;
    let file_size = file.metadata().await?.len();
    socket.write_all(&file_size.to_le_bytes()).await?;

    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB 缓冲区
    let mut total_sent = 0;
    let start_time = Instant::now(); // 添加开始时间记录

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        let chunk = &buffer[..n];
        let mut retries = 0;

        loop {
            // 发送分块大小和数据
            if let Err(e) = timeout(
                Duration::from_secs(5),
                socket.write_all(&(n as u32).to_le_bytes()),
            )
            .await
            {
                eprintln!("Failed to send chunk size: {:?}", e);
                break;
            }
            if let Err(e) = timeout(Duration::from_secs(5), socket.write_all(chunk)).await {
                eprintln!("Failed to send chunk data: {:?}", e);
                break;
            }

            // 发送校验和
            let checksum = calculate_checksum(chunk);
            if let Err(e) = timeout(Duration::from_secs(5), socket.write_all(&[checksum])).await {
                eprintln!("Failed to send checksum: {:?}", e);
                break;
            }
            if let Err(e) = timeout(Duration::from_secs(5), socket.flush()).await {
                eprintln!("Failed to flush socket: {:?}", e);
                break;
            }

            // 等待 ACK
            let mut ack = [0u8; 1];
            match timeout(Duration::from_secs(5), socket.read_exact(&mut ack)).await {
                Ok(Ok(_)) if ack[0] == 1 => {
                    total_sent += n as u64;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = (total_sent as f64) / (1024.0 * 1024.0) / elapsed; // MB/s
                    println!(
                        "Progress: {:.2}% - Speed: {:.2} MB/s",
                        (total_sent as f64 / file_size as f64) * 100.0,
                        speed
                    );
                    break;
                }
                _ => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        return Err(anyhow::anyhow!("Max retries exceeded"));
                    }
                    eprintln!("Retrying chunk... (attempt {})", retries + 1);
                }
            }
        }
    }

    let total_time = start_time.elapsed().as_secs_f64();
    let avg_speed = (total_sent as f64) / (1024.0 * 1024.0) / total_time;
    println!(
        "File sent successfully! Average speed: {:.2} MB/s",
        avg_speed
    );
    Ok(())
}

fn calculate_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
}
