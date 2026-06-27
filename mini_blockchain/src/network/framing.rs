use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub async fn write_frame(stream: &mut TcpStream, data: &[u8]) -> Result<(), std::io::Error> {
    
    let length = data.len() as u32;
    let length_bytes = length.to_be_bytes();

    stream.write_all(&length_bytes).await?;
    stream.write_all(data).await?;

    Ok(())
}

pub async fn read_frame(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {

    const MAX_MESSAGE_SIZE: usize = 8_000_000;

    let mut buffer_size = [0u8; 4];
    stream.read_exact(&mut buffer_size).await?;

    let n = u32::from_be_bytes(buffer_size) as usize;

    if n> MAX_MESSAGE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "message too long"
        ));
    }

    let mut data: Vec<u8> = vec![0u8; n];

    stream.read_exact(&mut data).await?;

    

    Ok(data)
}