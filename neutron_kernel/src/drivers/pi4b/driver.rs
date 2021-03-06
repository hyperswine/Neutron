// Pi 4B Driver Support
// For UART, GIC, GPIO

// TODO: GET these working for neutron drivers instead

use crate::drivers::{DriverManager, NeutronDriver};

struct BSPDriverManager {
    device_drivers: [&'static NeutronDriver; 3],
}

static BSP_DRIVER_MANAGER: BSPDriverManager = BSPDriverManager {
    device_drivers: [
        &super::GPIO,
        &super::PL011_UART,
        &super::INTERRUPT_CONTROLLER,
    ],
};

pub fn driver_manager() -> &'static DriverManager {
    &BSP_DRIVER_MANAGER
}
