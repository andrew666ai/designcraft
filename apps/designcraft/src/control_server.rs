//! Loopback JSON-lines control server: one request per line, one reply per line.
//!
//! The first line on every connection must be `{"method":"auth","params":{"token":…}}`.
//! Nothing is dispatched before that succeeds. This is the transport `designcraft-cli mcp --connect` wraps.

use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

use designcraft_ui_egui::ControlRequest;
use serde_json::{Value, json};

pub fn start(port: u16, token: String, ctx: egui::Context) -> Receiver<ControlRequest> {
    let (tx, rx) = channel::<ControlRequest>();
    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("designcraft: control server failed to bind 127.0.0.1:{port}: {e}");
            return rx;
        }
    };
    eprintln!("designcraft: control server listening on 127.0.0.1:{port}");
    if let Err(e) = std::thread::Builder::new().name("designcraft-control-listen".into()).spawn(move || {
        if let Err(e) = designcraft_mcp::accept_authenticated(listener, token, move |method, params| dispatch(&tx, &ctx, method, params)) {
            eprintln!("designcraft: control server stopped: {e}");
        }
    }) {
        eprintln!("designcraft: control server failed to start: {e}");
    }
    rx
}

fn dispatch(tx: &Sender<ControlRequest>, ctx: &egui::Context, method: String, params: Value) -> Value {
    let (req, rrx) = ControlRequest::new(method, params);
    if tx.send(req).is_err() {
        return json!({"ok": false, "error": "control channel closed"});
    }
    ctx.request_repaint();
    rrx.recv_timeout(Duration::from_secs(60)).unwrap_or_else(|_| json!({"ok": false, "error": "timeout"}))
}
