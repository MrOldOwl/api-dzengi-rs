#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecvWindowRequest {
    pub recv_window: Option<u64>,
}
impl RecvWindowRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_recv_window(mut self, recv_window: Option<u64>) -> Self {
        self.recv_window = recv_window;
        self
    }
}
