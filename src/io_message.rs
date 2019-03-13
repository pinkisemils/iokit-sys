// exporting IOMessage.h

type IOMessage = u32;

// mach/error.h
// 84:#define err_sub(x)           (((x)&0xfff)<<14)
macro_rules! err_sub {
    ($err_num: expr) => {
        ($err_num &0xfff) << 14
    }
}

// mach/error.h:83
// err_system(x) = ((signed)((((unsigned)(x))&0x3f)<<26);
macro_rules! err_system {
    ($err_num: expr) => {
        (((($err_num as u32)&0x3f)<<26) as i32)
    }
}


// IOReturn.h:48
// iokit_common_msg(message) = (UInt32)(sys_iokit|sub_iokit_common|message)
//
// sys_iokit = err_system(0x38);
// err_system(x) = ((signed)((((unsigned)(x))&0x3f)<<26);
//
// sub_iokit_common = err_sub(0)
// err_sub(x) = (((x)&0xfff)<<14)
macro_rules! iokit_common_msg {
    ($msg: expr) => {
        (err_system!(0x38) | err_sub!(0) | $msg) as IOMessage
    }
}

pub const kIOMessageServiceIsTerminated: IOMessage = iokit_common_msg!(0x010);
pub const kIOMessageServiceIsSuspended: IOMessage = iokit_common_msg!(0x020);
pub const kIOMessageServiceIsResumed: IOMessage = iokit_common_msg!(0x030);
pub const kIOMessageServiceIsRequestingClose: IOMessage = iokit_common_msg!(0x100);
pub const kIOMessageServiceIsAttemptingOpen: IOMessage = iokit_common_msg!(0x101);
pub const kIOMessageServiceWasClosed: IOMessage = iokit_common_msg!(0x110);
pub const kIOMessageServiceBusyStateChange: IOMessage = iokit_common_msg!(0x120);
pub const kIOMessageConsoleSecurityChange: IOMessage = iokit_common_msg!(0x128);
pub const kIOMessageServicePropertyChange: IOMessage = iokit_common_msg!(0x130);
pub const kIOMessageCopyClientID: IOMessage = iokit_common_msg!(0x330);
pub const kIOMessageSystemCapabilityChange: IOMessage = iokit_common_msg!(0x340);
pub const kIOMessageDeviceSignaledWakeup: IOMessage = iokit_common_msg!(0x350);
pub const kIOMessageDeviceWillPowerOff: IOMessage = iokit_common_msg!(0x210);
pub const kIOMessageDeviceHasPoweredOn: IOMessage = iokit_common_msg!(0x230);
pub const kIOMessageSystemWillPowerOff: IOMessage = iokit_common_msg!(0x250);
pub const kIOMessageSystemWillRestart: IOMessage = iokit_common_msg!(0x310);
pub const kIOMessageSystemPagingOff: IOMessage = iokit_common_msg!(0x255);
pub const kIOMessageCanSystemSleep: IOMessage = iokit_common_msg!(0x270);
pub const kIOMessageSystemWillNotSleep: IOMessage = iokit_common_msg!(0x290);
pub const kIOMessageSystemWillSleep: IOMessage = iokit_common_msg!(0x280);
pub const kIOMessageSystemWillPowerOn: IOMessage = iokit_common_msg!(0x320);
pub const kIOMessageSystemHasPoweredOn: IOMessage = iokit_common_msg!(0x300);
pub const kIOMessageCanDevicePowerOff: IOMessage = iokit_common_msg!(0x200);
pub const kIOMessageDeviceWillNotPowerOff: IOMessage = iokit_common_msg!(0x220);
pub const kIOMessageSystemWillNotPowerOff: IOMessage = iokit_common_msg!(0x260);
pub const kIOMessageCanSystemPowerOff: IOMessage = iokit_common_msg!(0x240);
pub const kIOMessageDeviceWillPowerOn: IOMessage = iokit_common_msg!(0x215);
pub const kIOMessageDeviceHasPoweredOff: IOMessage = iokit_common_msg!(0x225);
