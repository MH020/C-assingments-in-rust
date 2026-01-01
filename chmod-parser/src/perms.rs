use core::panic;

pub struct NyPermsT {
    pub bits: u32,
}

pub const USER_READ: u32    = 0o400;
pub const USER_WRITE: u32   = 0o200;
pub const USER_EXECUTE: u32 = 0o100;

pub const GROUP_READ: u32    = 0o040;
pub const GROUP_WRITE: u32   = 0o020;
pub const GROUP_EXECUTE: u32 = 0o010;

pub const OTHER_READ: u32    = 0o004;
pub const OTHER_WRITE: u32   = 0o002;
pub const OTHER_EXECUTE: u32 = 0o001;

impl NyPermsT {

    pub fn chmod(&mut self, input: String) {

        let op = input.chars().find(|&c| c == '+' || c == '-');

        let (who, perm_chars) = if let Some((l, r)) = input.split_once('+') {
            (l, r)
        } else if let Some((l, r)) = input.split_once('-') {
            (l, r)
        } else {
            panic!("wrong chmod input")
        };

        for chars in perm_chars.chars() {
            let mask = match (who, chars) {
                ("u", 'r') => USER_READ,
                ("u", 'w') => USER_WRITE,
                ("u", 'x') => USER_EXECUTE,
                ("g", 'r') => GROUP_READ,
                ("g", 'w') => GROUP_WRITE,
                ("g", 'x') => GROUP_EXECUTE,
                ("o", 'r') => OTHER_READ,
                ("o", 'w') => OTHER_WRITE,
                ("o", 'x') => OTHER_EXECUTE,
                _ => 0,
            };
            if op == Some('+') {
                self.bits |= mask;
            } else {
                self.bits &= !mask;
            }
        }
    }
}
