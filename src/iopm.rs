use super::{
    io_connect_t, io_service_t, IONotificationPortRef, IOReturn, IOServiceInterestCallback, io_object_t,
};
use cf::{array::CFArrayRef, string::CFStringRef, date::CFDateRef};
use mach::port::mach_port_t;
use std::ffi::c_void;
use libc::{intptr_t};

// exports from #import <IOKit/pwr_mgt/IOPMLib.h>

extern "C" {
    pub fn IOPMFindPowerManagement(master_device_port: mach_port_t) -> io_connect_t;

    pub fn IOPMSetAggressiveness(
        fb: io_connect_t,
        factor_type: u32,
        aggressiveness: *mut u32,
    ) -> IOReturn;

    pub fn IOPMGetAggressiveness(
        fb: io_connect_t,
        factor_type: u32,
        aggressiveness: *mut u32,
    ) -> IOReturn;

    pub fn IOPMSleepEnabled() -> bool;

    pub fn IOPMSleepSystem(fb: io_connect_t) -> IOReturn;
    pub fn IOPMCopyBatteryInfo(masterPort: mach_port_t, info: *mut CFArrayRef) -> IOReturn;

    pub fn IORegisterApp(
        refcon: *mut c_void,
        theDriver: io_service_t,
        thePortRef: *mut IONotificationPortRef,
        callback: IOServiceInterestCallback,
    ) -> io_connect_t;

    pub fn IORegisterForSystemPower( refcon: *mut c_void, thePortRef: *mut IONotificationPortRef, callback: IOServiceInterestCallback, notifier: *mut io_object_t) -> io_connect_t;

    pub fn IODeregisterApp( notifier: *mut io_object_t ) -> IOReturn;

    pub fn IOAllowPowerChange( kernelPort: io_connect_t, notificationID: intptr_t ) -> IOReturn;
    pub fn IOCancelPowerChange( kernelPort: io_connect_t, notificationID: intptr_t ) -> IOReturn;

    pub fn IOPMSchedulePowerEvent(time_to_wake: CFDateRef, my_id: CFStringRef, power_event_type: CFStringRef ) -> IOReturn;
    pub fn IOPMCancelScheduledPowerEvent(time_to_wake: CFDateRef, my_id: CFStringRef, power_event_type: CFStringRef) -> IOReturn;

    pub fn IOPMCopyScheduledPowerEvents() -> CFArrayRef;
}
