#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Command {
    StartRecording,
    StopRecording,
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::StartRecording => "START_RECORDING",
            Command::StopRecording => "STOP_RECORDING",
        }
    }
}
