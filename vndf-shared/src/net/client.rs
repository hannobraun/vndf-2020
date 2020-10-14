use crate::net::{self, msg};

pub type Conn = net::Conn<msg::FromServer, msg::FromClient>;
