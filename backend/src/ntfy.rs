use std::time::Duration;

use tracing::warn;

#[derive(Clone, Debug)]
pub struct NtfyClient {
    base_url: String,
    client: reqwest::Client,
}

impl NtfyClient {
    pub fn new(base_url: &str) -> Self {
        let base_url = base_url.trim().to_string();

        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
        {
            Ok(client) => client,
            Err(err) => {
                warn!(error = %err, "failed to build reqwest client; falling back to default");
                reqwest::Client::new()
            }
        };

        Self { base_url, client }
    }

    pub async fn send(&self, message: &str, title: &str, priority: u8, tags: &str) {
        if self.base_url.is_empty() {
            warn!("ntfy base_url is empty; skipping send");
            return;
        }

        let mut req = self.client.post(&self.base_url).body(message.to_string());

        if !title.is_empty() {
            req = req.header("Title", title);
        }

        if priority > 0 {
            req = req.header("Priority", priority.to_string());
        }

        if !tags.is_empty() {
            req = req.header("Tags", tags);
        }

        match req.send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    warn!(status = %resp.status(), "ntfy send failed");
                }
            }
            Err(err) => {
                warn!(error = %err, "ntfy send error");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn ntfy_send_writes_headers_and_body() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}/schweinehund", addr);

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0u8; 8192];
            let mut n = 0usize;
            loop {
                let read = socket.read(&mut buf[n..]).await.unwrap();
                if read == 0 {
                    break;
                }
                n += read;
                if n >= 4 && buf[..n].windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
                if n == buf.len() {
                    break;
                }
            }

            let request_head = String::from_utf8_lossy(&buf[..n]).to_string();
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
            socket.write_all(response.as_bytes()).await.unwrap();
            request_head
        });

        let client = NtfyClient::new(&url);
        client.send("hello", "Hello", 4, "tag1,tag2").await;

        let req = server.await.unwrap();
        let lower = req.to_lowercase();

        assert!(lower.contains("post /schweinehund"));
        assert!(lower.contains("title: hello"));
        assert!(lower.contains("priority: 4"));
        assert!(lower.contains("tags: tag1,tag2"));
    }

    #[tokio::test]
    async fn ntfy_send_is_best_effort_on_connection_error() {
        let client = NtfyClient::new("http://127.0.0.1:1/schweinehund");
        client.send("msg", "t", 1, "").await;
    }
}
