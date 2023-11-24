use bgsp_lib2::bgsp_common::PATTERN_SIZE;

pub const SP_PATTERN_TBL: [Option<(u32, u32, &[u64])>; 64] = {
    let mut tbl: [Option<(u32, u32, &[u64])>; 64] = [None; 64];
    let mut tbl_pos = 0;
    let mut idx = 0;
    while idx < SP_CHARS_1X1.len() {
        tbl[tbl_pos] = Some((1, 1, &SP_CHARS_1X1[idx]));
        tbl_pos += 1;
        idx += 1;
    }
    tbl
};

const SP_CHARS_1X1: &[[u64; PATTERN_SIZE * 1 * 1]] = &[
    [
        0x0000005151000000,
        0x0000514f4f510000,
        0x00514f4f4f4f5100,
        0x514f4f4f4f4f4f51,
        0x514f4f4f4f4f4f51,
        0x00514f4f4f4f5100,
        0x0000514f4f510000,
        0x0000005151000000,
    ],
    [
        0x0000007f7f000000,
        0x00007f7f7f7f0000,
        0x007f7f00007f7f00,
        0x7f7f007f7f007f7f,
        0x0000007f7f000000,
        0x0000007f7f000000,
        0x0000000000000000,
        0x0000000000000000,
    ],
];
