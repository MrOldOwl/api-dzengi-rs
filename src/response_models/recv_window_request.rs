#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecvWindowRequest {
    pub recv_window: u64,
}
impl Default for RecvWindowRequest {
    fn default() -> Self {
        Self { recv_window: 5000 }
    }
}
impl RecvWindowRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}
