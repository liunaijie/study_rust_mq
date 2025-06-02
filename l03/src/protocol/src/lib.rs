use common_base::protocol_util;

pub fn protocol_fnc(par: u32) -> String {
    protocol_util::get_version(par)
}
