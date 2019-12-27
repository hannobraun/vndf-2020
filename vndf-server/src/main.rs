use vndf_shared::{
    Server,
    net,
};


fn main() -> net::Result {
    let mut server = Server::start_default()?;

    loop {
        server.update();
    }
}
