use std::sync::Arc;

use tracing::warn;

#[derive(Clone, Debug)]
pub struct NtfyClient {
    base_url: String,
    agent: Arc<ureq::Agent>,
}

impl NtfyClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim().to_string(),
            agent: Arc::new(ureq::Agent::new_with_defaults()),
        }
    }

    pub async fn send(&self, message: &str, title: &str, priority: u8, tags: &str) {
        if self.base_url.is_empty() {
            warn!("ntfy base_url is empty; skipping send");
            return;
        }

        let agent = self.agent.clone();
        let url = self.base_url.clone();
        let message = message.to_string();
        let title = title.to_string();
        let tags = tags.to_string();

        let _ = tokio::task::spawn_blocking(move || {
            let mut req = agent.post(&url);

            if !title.is_empty() {
                req = req.header("Title", &title);
            }
            if priority > 0 {
                req = req.header("Priority", &priority.to_string());
            }
            if !tags.is_empty() {
                req = req.header("Tags", &tags);
            }

            match req.send(message) {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if status >= 400 {
                        warn!(status, "ntfy send failed");
                    }
                }
                Err(err) => warn!(error = %err, "ntfy send error"),
            }
        })
        .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ntfy_send_is_best_effort_on_connection_error() {
        let client = NtfyClient::new("http://127.0.0.1:1/schweinehund");
        client.send("msg", "t", 1, "").await;
    }
}
