#[repr(C)]
#[derive(Debug, Clone)]
pub struct HdDriveId {
    config: u16,
    cyls: u16,
    reserved2: u16,
    heads: u16,
    track_bytes: u16,
    sector_bytes: u16,
    sectors: u16,
    vendor: [u16; 3],
    serial_no: [char; 20],
    buf_type: u16,
    buf_size: u16,

    ecc_bytes: u16,
    fw_rev: [char; 8],
    model: [char; 40],
    max_multsec: char,
    vendor3: char,
    dword_io: u16,
    vendor4: char,
    capability: char,

    reserved50: u16,
    vendor5: char,
    t_pio: char,
    vendor6: char,
    t_dma: char,
    field_valid: u16,
    cur_cyls: u16,
    cur_heads: u16,
    cur_sectors: u16,
    cur_capacity: [u16; 2],
    multsect: char,
    multsect_valid: char,
    lba_capacity: i32,
    dma_1word: u16,
    dma_mword: u16,
    eide_pio_modes: u16,
    eide_dma_min: u16,
    eide_dma_time: u16,
    eide_pio: u16,
    eide_pio_iordy: u16,
    words69_70: [u16; 2],

    words71_74: [u16; 4],

    queue_depth: u16,

    words76_79: [u16; 4],
    major_rev_num: u16,
    minor_rev_num: u16,
    command_set_1: u16,

    command_set_2: u16,

    cfsse: u16,

    cfs_enable_1: u16,

    cfs_enable_2: u16,

    csf_default: u16,

    dma_ultra: u16,
    trseuc: u16,
    trs_euc: u16,
    cur_apm_values: u16,
    mprc: u16,
    hw_config: u16,

    acoustic: u16,

    msrqs: u16,
    sxfert: u16,
    sal: u16,
    spg: i32,
    lba_capacity_2: u64,
    words104_125: [u16; 22],
    last_lun: u16,
    word127: u16,

    dlf: u16,

    csfo: u16,

    words130_155: [u16; 26],
    word156: u16,
    words157_159: [u16; 3],
    cfa_power: u16,

    words161_175: [u16; 15],
    words176_205: [u16; 30],
    words206_254: [u16; 49],
    integrity_word: u16,
}
