use super::is_binary32_nan;

pub(in crate::simulation::persistence) fn cap_alpha_one_masked_daz_ftz_off(alpha: u32) -> u32 {
    if is_binary32_nan(alpha) || (alpha & 0x80000000 == 0 && alpha >= 0x3f800000) {
        return 0x3f800000;
    }
    alpha
}
