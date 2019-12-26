use vndf_shared::net::{
    Conn,
    Error,
    Server,
    conn,
    server,
};


#[test]
fn server_should_emit_connect_events() -> Result<(), Error> {
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
