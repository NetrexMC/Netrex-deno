use std::{collections::VecDeque, sync::Arc};

use mcpe_protocol::mcpe::Packet;
use tokio::sync::mpsc::{error::SendError, Sender};

#[derive(Debug, Clone)]
pub enum SessionCommand {
    /// Not immediate
    Send(Packet),
    /// Immediate
    SendStream(Vec<u8>),
    /// Immediate
    Disconnect(String),
}

/// A network session keeps track of incoming and outgoing packets
/// This is mainly a proxy for the Server to better handle packets.
#[derive(Clone)]
pub struct Session {
    /// The address of the connection
    address: String,
    /// The packets that are queued, but not immediately sent.
    packets: VecDeque<Packet>,
    /// The sender to send packets to the client.
    /// This is a channel that is used to send packets to the client immediately.
    sender: Arc<Sender<(String, SessionCommand)>>,
}

impl Session {
    /// Create a new session for a new connection.
    /// This will create a new sender to send packets to the client.
    pub fn new(address: String, sender: Arc<Sender<(String, SessionCommand)>>) -> Self {
        Self {
            address,
            packets: VecDeque::new(),
            sender,
        }
    }
    /// Disconnect the session
    /// This will permanently remove the session from the server.
    pub async fn disconnect<T: Into<String>>(&self, reason: T) {
        self.dispatch(SessionCommand::Disconnect(reason.into()))
            .await
            .unwrap();
    }

    /// Ticks the session, this is called every tick
    /// This is used to send packets to the client
    pub async fn tick(&mut self) {
        // foreach packet in the packets queue, send it.
        // Packets should be batched and compressed here, but for now,
        // We just send them all at once.
        for packet in self.packets.clone().drain(..) {
            // we don't care about this error.
            self.dispatch(SessionCommand::Send(packet)).await;
        }
        self.packets.clear();
    }

    /// Send a packet to the client
    /// If immediate is true, the packet will be sent immediately, completely skipping the queue.
    pub async fn send(&mut self, packet: Packet, immediate: bool) {
        if immediate {
            self.dispatch(SessionCommand::Send(packet)).await;
        } else {
            self.packets.push_back(packet);
        }
    }

    /// Immediately sends any buffer to the client
    pub async fn send_stream(&self, stream: Vec<u8>) {
        self.dispatch(SessionCommand::SendStream(stream)).await;
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    async fn dispatch(
        &self,
        command: SessionCommand,
    ) -> Result<(), SendError<(String, SessionCommand)>> {
        return self.sender.send((self.address.clone(), command)).await;
    }
}
