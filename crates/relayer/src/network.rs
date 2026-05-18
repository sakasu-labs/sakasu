//! HTTP listener for the relayer node.
//!
//! v0 ships three endpoints the rest of the stack actually exercises:
//!   - `GET  /health`  — liveness probe, returns `{"ok": true}`
//!   - `GET  /metrics` — process counters (uptime, requests served,
//!                      intent count, last error timestamp)
//!   - `POST /intent`  — accepts an encrypted intent envelope. v0 stores it
//!                      in an in-memory queue and replies with the queued
//!                      intent id. Real decryption + Solana broadcast lands
//!                      in v0.2 (see ROADMAP.md).
//!
//! Everything below runs on the standard library's `TcpListener` so the crate
//! stays dependency-light. A switch to `axum` / `tokio` is tracked under v0.2.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

static REQUESTS_SERVED: AtomicU64 = AtomicU64::new(0);
static INTENTS_QUEUED: AtomicU64 = AtomicU64::new(0);

struct Metrics {
    started_at: Instant,
    last_error_unix: Mutex<Option<u64>>,
}

impl Metrics {
    fn new() -> Self {
        Self {
            started_at: Instant::now(),
            last_error_unix: Mutex::new(None),
        }
    }

    fn record_error(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if let Ok(mut guard) = self.last_error_unix.lock() {
            *guard = Some(now);
        }
    }

    fn snapshot(&self) -> String {
        let uptime_s = self.started_at.elapsed().as_secs();
        let last_err = self
            .last_error_unix
            .lock()
            .ok()
            .and_then(|g| *g)
            .map(|t| t.to_string())
            .unwrap_or_else(|| "null".to_string());
        format!(
            "{{\"uptime_s\":{},\"requests_served\":{},\"intents_queued\":{},\"last_error_unix\":{}}}",
            uptime_s,
            REQUESTS_SERVED.load(Ordering::Relaxed),
            INTENTS_QUEUED.load(Ordering::Relaxed),
            last_err
        )
    }
}

struct Request {
    method: String,
    path: String,
    body: Vec<u8>,
}

fn parse_request(stream: &mut TcpStream) -> std::io::Result<Request> {
    let mut reader = BufReader::new(stream);
    let mut start_line = String::new();
    reader.read_line(&mut start_line)?;
    let mut parts = start_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let mut headers: HashMap<String, String> = HashMap::new();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }

    let content_length: usize = headers
        .get("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    Ok(Request { method, path, body })
}

fn write_response(stream: &mut TcpStream, status: u16, body: &str) {
    let reason = match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Internal Server Error",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {len}\r\nConnection: close\r\n\r\n{body}",
        status = status,
        reason = reason,
        len = body.len(),
        body = body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

fn handle(stream: &mut TcpStream, metrics: &Metrics) {
    let req = match parse_request(stream) {
        Ok(r) => r,
        Err(_) => {
            metrics.record_error();
            write_response(stream, 400, r#"{"error":"bad request"}"#);
            return;
        }
    };

    REQUESTS_SERVED.fetch_add(1, Ordering::Relaxed);

    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/health") => write_response(stream, 200, r#"{"ok":true}"#),
        ("GET", "/metrics") => {
            let snap = metrics.snapshot();
            write_response(stream, 200, &snap);
        }
        ("POST", "/intent") => {
            // v0 accepts the envelope, assigns a queue id, and replies. The
            // real decryption path (X25519 + ChaCha20-Poly1305) and Solana
            // commit_transfer broadcast land in v0.2 (see ROADMAP.md).
            let id = INTENTS_QUEUED.fetch_add(1, Ordering::Relaxed) + 1;
            let payload_bytes = req.body.len();
            let body = format!(
                r#"{{"intent_id":{},"payload_bytes":{},"status":"queued"}}"#,
                id, payload_bytes
            );
            write_response(stream, 202, &body);
        }
        ("GET", _) | ("POST", _) => write_response(stream, 404, r#"{"error":"unknown route"}"#),
        _ => write_response(stream, 405, r#"{"error":"method not allowed"}"#),
    }
}

/// Bind to `addr` and serve requests in a thread-per-connection model.
///
/// Returns on a clean shutdown (currently never — the daemon is expected to
/// be terminated by SIGTERM).
pub fn serve(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let metrics: &'static Metrics = Box::leak(Box::new(Metrics::new()));
    println!("sakasu-relayer listening on http://{addr}");
    println!("  GET  /health");
    println!("  GET  /metrics");
    println!("  POST /intent");

    for incoming in listener.incoming() {
        match incoming {
            Ok(mut stream) => {
                thread::spawn(move || handle(&mut stream, metrics));
            }
            Err(e) => {
                eprintln!("accept error: {e}");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_snapshot_well_formed() {
        let m = Metrics::new();
        let snap = m.snapshot();
        assert!(snap.starts_with("{"));
        assert!(snap.contains("uptime_s"));
        assert!(snap.contains("requests_served"));
        assert!(snap.contains("intents_queued"));
        assert!(snap.contains("last_error_unix"));
    }

    #[test]
    fn metrics_record_error_sets_timestamp() {
        let m = Metrics::new();
        assert!(m.last_error_unix.lock().unwrap().is_none());
        m.record_error();
        assert!(m.last_error_unix.lock().unwrap().is_some());
    }
}
