// src/offline_didcomm.rs

pub struct OfflineDidCommSession {
    // All secrets remain enclave-side; this struct only holds handles.
    pub local_did: String,
    pub remote_did: String,
    pub session_id: [u8; 16],
    pub established_at: std::time::Instant,
}

impl OfflineDidCommSession {
    // Called after enclave has done mutual auth and derived sk.
    pub fn new(local_did: String, remote_did: String, session_id: [u8; 16]) -> Self {
        Self {
            local_did,
            remote_did,
            session_id,
            established_at: std::time::Instant::now(),
        }
    }
}
