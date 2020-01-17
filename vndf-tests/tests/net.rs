use vndf_server::net::{
    Error,
    Event,
    Network,
};
use vndf_shared::net::{
    self,
    client::Conn,
    msg,
};


#[test]
fn network_should_emit_receive_events() -> net::Result {
    let mut server = Network::start_local()?;
    let mut conn   = Conn::connect(server.addr())?;

    let sent = msg::FromClient::Hello;
    conn.send(sent)?;

    let mut received = None;

    while received.is_none() {
        for event in server.events() {
            if let Event::Message(id, message) = event {
                received = Some((id, message));
            }
        }
    }

    assert_eq!(received, Some((conn.local_addr, sent)));

    Ok(())
}

#[test]
fn network_should_report_client_errors() -> net::Result {
    let mut server = Network::start_local()?;
    let     client = Conn::connect(server.addr())?;
    let     addr   = client.local_addr;

    client.disconnect();

    let mut disconnect_id = None;
    while disconnect_id.is_none() {
        // Attempt to send, to trigger an error.
        server.send(addr, msg::FromServer::Ping);

        for event in server.events() {
            if let Event::Error(id, _error) = event {
                disconnect_id = Some(id);
            }
        }
    }

    assert_eq!(Some(addr), disconnect_id);

    Ok(())
}

#[test]
fn clients_should_emit_receive_events() -> Result<(), Error> {
    let mut server = Network::start_local()?;
    let mut client = Conn::connect(server.addr())?;

    client.send(msg::FromClient::Hello)?;

    let mut client_connected = false;
    while !client_connected {
        for event in server.events() {
            if let Event::Message(_, _) = event {
                client_connected = true;
            }
        }
    }

    let message = msg::FromServer::Ping;

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
