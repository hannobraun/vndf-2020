pub use vndf_shared as shared;


use self::shared::{
    Server,
    net,
};


fn main() -> net::Result {
    let mut server = Server::start_default()?;

    loop {
        server.update();
    }
}
