use vndf_shared::net::{
    self,
    Server,
    client::Conn,
    conn,
    msg,
    server,
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

    assert!(events.contains(&Ok(server::Event::Connect(conn::Id(0)))));
    assert!(events.contains(&Ok(server::Event::Connect(conn::Id(1)))));

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
            if let server::Event::Message(message) = event? {
                messages.push(message);
            }
        }
    }

    assert!(messages.contains(&message));

    Ok(())
}
