use toadster::handle;

use super::Health;

#[derive(Debug)]
pub struct Death {
    pub handle: handle::Strong<Health>,
}
