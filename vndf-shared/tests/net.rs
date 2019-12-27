use vndf_shared::net::{
    self,
    Server,
    client::Conn,
    msg,
    server::{
        self,
        ConnId,
    },
};


#[test]
fn server_should_emit_connect_events() -> net::Result {
    let mut server = Server::start_local()?;

    Conn::connect(server.addr())?;
    Conn::connect(server.addr())?;

    let mut events = Vec::new();

    while events.len() < 2 {
        for event in server.events() {
            events.push(event);
        }
    }

    assert!(events.contains(&server::Event::Connect(ConnId(0))));
    assert!(events.contains(&server::Event::Connect(ConnId(1))));

    Ok(())
}

#[test]
fn server_should_emit_receive_events() -> net::Result {
    let mut server = Server::start_local()?;

    let message = msg::FromClient::Hello;
    Conn::connect(server.addr())?
        .send(message)?;

    let mut messages = Vec::new();

    while messages.len() < 1 {
        for event in server.events() {
            if let server::Event::Message(message) = event {
                messages.push(message);
            }
        }
    }

    assert!(messages.contains(&message));

    Ok(())
}

#[test]
fn clients_should_emit_receive_events() -> Result<(), server::Error> {
    let mut server = Server::start_local()?;
    let mut client = Conn::connect(server.addr())?;

    let message = msg::FromServer::Welcome;

    let mut client_id = None;
    while client_id.is_none() {
        for event in server.events() {
            if let server::Event::Connect(id) = event {
                client_id = Some(id);
            }
        }
    }

    if let Some(id) = client_id {
        // This is going to happen, otherwise the previous loop wouldn't have
        // finished.
        server.send(id, message)?;
    }

    let mut messages = Vec::new();
    while messages.len() < 1 {
        for message in client.incoming() {
            messages.push(message?);
        }
    }

    assert!(messages.contains(&message));

    Ok(())
}
