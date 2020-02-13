use toadster::StrongHandle;

use super::Health;


pub struct Death {
    pub handle: StrongHandle<Health>,
}
