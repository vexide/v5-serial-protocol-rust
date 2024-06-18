use super::cdc2::{Cdc2CommandPacket, Cdc2ReplyPacket};

pub struct Log {
    /// (RESEARCH NEEDED)
    pub code: u8,
    
    /// The subtype under the description (RESEARCH NEEDED)
    pub log_type: u8,

    /// The type of the log message (RESEARCH NEEDED)
    pub description: u8,

    /// (RESEARCH NEEDED)
    pub spare: u8,

    /// How long (in milliseconds) after the brain powered on
    pub time: u16,
}

pub type GetLogCountPacket = Cdc2CommandPacket<0x56, 0x24, ()>;
pub type GetLogCountReplyPacket = Cdc2ReplyPacket<0x56, 0x24, GetLogCountReplyPayload>;

pub struct GetLogCountReplyPayload {
    pub ignored: u8,
    pub count: u32,
}

/// For example: If the brain has 26 logs, from A to Z. With offset 5 and count 5, it returns [V, W, X, Y, Z]. With offset 10 and count 5, it returns [Q, R, S, T, U].
pub type ReadLogPagePacket = Cdc2CommandPacket<0x56, 0x25, ReadLogPagePayload>;
pub type ReadLogPageReplyPacket = Cdc2ReplyPacket<0x56, 0x25, ReadLogPageReplyPayload>;

pub struct ReadLogPagePayload {
    pub offset: u32,
    pub count: u32,
}

pub struct ReadLogPageReplyPayload {
    /// The offset number used in this packet.
    pub offset: u32,
    /// Number of elements in the following array.
    pub count: u32,
    pub entries: Vec<Log>,
}