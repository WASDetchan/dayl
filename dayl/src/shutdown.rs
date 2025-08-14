use tokio::sync::mpsc::UnboundedSender;

pub fn register_fence_shutdown() -> UnboundedSender<()> {
    let (send, mut recv) = tokio::sync::mpsc::unbounded_channel::<()>();
    tokio::spawn(async move {
        tokio::select! {
            _ = recv.recv() => {},
            _ = tokio::signal::ctrl_c() => {},
        }
        wknup::fence_shutdown();
    });
    send
}
