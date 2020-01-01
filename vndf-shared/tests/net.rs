use vndf_shared::net::{
    self,
    Server,
    client::Conn,
    msg,
    server,
};


#[test]
fn server_should_emit_connect_events() -> net::Result {
    let mut server = Server::start_local()?;

    let conn_1 = Conn::connect(server.addr())?;
    let conn_2 = Conn::connect(server.addr())?;

    let mut events = Vec::new();

    while events.len() < 2 {
        for event in server.events() {
            events.push(event);
        }
    }

    assert!(events.contains(&server::Event::Connect(conn_1.local_addr)));
    assert!(events.contains(&server::Event::Connect(conn_2.local_addr)));

    Ok(())
}

#[test]
fn server_should_emit_receive_events() -> net::Result {
    let mut server = Server::start_local()?;

    let sent = msg::FromClient::Hello;
    Conn::connect(server.addr())?
        .send(sent)?;

    let mut client_id = None;
    let mut received  = None;

    while client_id.is_none() || received.is_none() {
        for event in server.events() {
            if let server::Event::Connect(id) = event {
                client_id = Some(id);
            }
            if let server::Event::Message(id, message) = event {
                received = Some((id, message));
            }
        }
    }

    assert_eq!(received, Some((client_id.unwrap(), sent)));

    Ok(())
}

#[test]
fn server_should_remove_clients_that_cause_errors() -> net::Result {
    let mut server = Server::start_local()?;
    let     client = Conn::connect(server.addr())?;

    let mut connect_id = None;
    while connect_id.is_none() {
        for event in server.events() {
            if let server::Event::Connect(id) = event {
                connect_id = Some(id);
            }
        }
    }

    client.disconnect();

    let mut disconnect_id = None;
    while disconnect_id.is_none() {
        // Attempt to send, to trigger an error.
        server.send(connect_id.unwrap(), msg::FromServer::Welcome)
            .expect("Client should exist");

        for event in server.events() {
            if let server::Event::Disconnect(id, _error) = event {
                disconnect_id = Some(id);
            }
        }
    }

    assert_eq!(connect_id, disconnect_id);

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
