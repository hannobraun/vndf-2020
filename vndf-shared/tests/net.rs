use vndf_shared::net::{
    self,
    Network,
    client::Conn,
    msg,
    network,
};


#[test]
fn network_should_emit_connect_events() -> net::Result {
    let mut server = Network::start_local()?;

    let conn_1 = Conn::connect(server.addr())?;
    let conn_2 = Conn::connect(server.addr())?;

    let mut events = Vec::new();

    while events.len() < 2 {
        for event in server.events() {
            events.push(event);
        }
    }

    assert!(events.contains(&network::Event::Connect(conn_1.local_addr)));
    assert!(events.contains(&network::Event::Connect(conn_2.local_addr)));

    Ok(())
}

#[test]
fn network_should_emit_receive_events() -> net::Result {
    let mut server = Network::start_local()?;
    let mut conn   = Conn::connect(server.addr())?;

    let sent = msg::FromClient::Hello;
    conn.send(sent)?;

    let mut received = None;

    while received.is_none() {
        for event in server.events() {
            if let network::Event::Message(id, message) = event {
                received = Some((id, message));
            }
        }
    }

    assert_eq!(received, Some((conn.local_addr, sent)));

    Ok(())
}

#[test]
fn network_should_remove_clients_that_cause_errors() -> net::Result {
    let mut server = Network::start_local()?;
    let     client = Conn::connect(server.addr())?;
    let     addr   = client.local_addr;

    client.disconnect();

    let mut disconnect_id = None;
    while disconnect_id.is_none() {
        // Attempt to send, to trigger an error.
        server.send(addr, msg::FromServer::Welcome(addr));

        for event in server.events() {
            if let network::Event::Disconnect(id, _error) = event {
                disconnect_id = Some(id);
            }
        }
    }

    assert_eq!(Some(addr), disconnect_id);

    Ok(())
}

#[test]
fn clients_should_emit_receive_events() -> Result<(), network::Error> {
    let mut server = Network::start_local()?;
    let mut client = Conn::connect(server.addr())?;

    client.send(msg::FromClient::Hello)?;

    let mut client_connected = false;
    while !client_connected {
        for event in server.events() {
            if let network::Event::Message(_, _) = event {
                client_connected = true;
            }
        }
    }

    let message = msg::FromServer::Welcome(client.peer_addr);

    server.send(client.local_addr, message);

    let mut messages = Vec::new();
    while messages.len() < 1 {
        for message in client.incoming() {
            messages.push(message?);
        }
    }

    assert!(messages.contains(&message));

    Ok(())
}
