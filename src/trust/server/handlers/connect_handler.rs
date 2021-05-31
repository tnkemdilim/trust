use super::IncomingChatMessage;
use crate::trust::server::ChatServer;
use crate::trust::server::ChatServerError;
use actix::Recipient;
use actix::{Context, Handler};

/// Connect a client message.
#[derive(actix::Message)]
#[rtype(result = "Result<String, ChatServerError>")]
pub struct Connect {
    pub addr: Recipient<IncomingChatMessage>,
}

/// Handler for Connect message.
impl Handler<Connect> for ChatServer {
    type Result = Result<String, ChatServerError>;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone just connected!!!");
        self.handle_new_connection(msg.addr)
    }
}
