use toadster::handle;

use super::Health;


pub struct Death {
    pub handle: handle::Strong<Health>,
}
