use serde::{Deserialize, Serialize};

pub const SERVER_PORT: &str = "9000";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SessionID(String);

impl SessionID {
    pub fn new(inner: String) -> Self {
        SessionID(inner)
    }
    pub fn inner(self) -> String {
        self.0
    }
}

impl From<&str> for SessionID {
    fn from(session_id: &str) -> Self {
        SessionID(session_id.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UserID(String);

impl UserID {
    pub fn new(inner: String) -> Self {
        UserID(inner)
    }
    pub fn inner(self) -> String {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalEnum {
    NewUser(UserID),

    SessionNew,
    SessionReady(SessionID),
    SessionJoin(SessionID),
    SessionJoinSuccess(SessionID),
    SessionJoinError(SessionID),

    VideoOffer(String, SessionID),
    VideoAnswer(String, SessionID),
    IceCandidate(String, SessionID),
    ICEError(String, SessionID),

    Debug,
}