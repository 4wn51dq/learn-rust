use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

/// TCP is a byte stream with no message boundaries, Length Prefix Framing solves it by 
/// writing the length of data into the stream prefix so the receiver gets to know when
/// to stop reading.
 

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


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::test;

    #[tokio::test]
    async fn test_tcp_roundtrip() {
        let data = "wsup";

        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        let server = tokio::spawn(async move {
            let (mut server_stream, socket_addr) = listener.accept().await.unwrap();
            read_frame(&mut server_stream).await.unwrap()
        });

        let client = tokio::spawn(async move {
            let mut client_stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
            write_frame(&mut client_stream, data.as_bytes()).await.unwrap();
        });
        

        let (result,_) = tokio::join!(
            server,
            client,
        );
        assert_eq!(data.as_bytes(), result.unwrap());
    }
}