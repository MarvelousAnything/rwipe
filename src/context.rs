use crate::utils::hdregs::HdDriveId;
use crate::utils::fstat::FileStat;
use crate::prng::{Prng, RwipeEntropy, PrngState};

pub enum DeviceType {
    Unknown = 0,
    Ide,
    Scsi,
    Compaq,
    Usb,
    Ieee1394,
}

pub enum PassType {
    None = 0,
    Write,
    Verify,
    FinalBlank,
    FinalOps2,
}

pub enum SelectType {
    None,
    True,
    TrueParent,
    False,
    FalseChild,
    Disabled,
}

// TODO: Figure out what I need to do with the speedring
pub struct SpeedringType {}

pub struct RwipeContext<'a> {
   pub block_size: i32,
   pub device_bus: i32,
   pub device_fd: i32,
   pub device_host: i32,
   pub device_id: HdDriveId,
   pub device_lun: i32,
   pub device_major: i32,
   pub device_minor: i32,
   pub device_part: i32,
   pub device_name: String,
   pub device_size: u64,
   pub device_stat: FileStat,
   pub device_type: DeviceType,
   pub device_target: i32,
   pub eta: u64,
   pub entropy_fd: u64,
   pub label: &'a str,
   pub pass_count: i32,
   pub pass_errors: u64,
   pub pass_size: u64,
   pub pass_type: PassType,
   pub pass_working: i32,
   pub pid: i32,
   pub prng: Prng<'a>,
   pub prng_seed: RwipeEntropy,
   pub prng_state: Box<dyn PrngState + 'a>,
   pub result: i32,
   pub round_count: i32,
   pub round_done: u64,
   pub round_errors: u64,
   pub round_size: u64,
   pub round_percent: f64,
   pub round_working: i32,
   pub sector_size: i32,
   pub select: SelectType,
   pub signal: i32,
   // dwipe_speedring_t speedring,
   pub status: i32,
   pub sync_status: i16,
   pub throughput: u64,
   pub verify_errors: u64

}
