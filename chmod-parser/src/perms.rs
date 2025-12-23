use core::panic;

pub struct NyPermsT {
    pub bits: u32,
}

pub const USER_READ: u32 = 0b000001;
pub const USER_WRITE: u32 = 0b000010;
pub const USER_EXECUTE: u32 = 0b000100;

pub const GROUP_READ: u32 = 0b001000;
pub const GROUP_WRITE: u32 = 0b010000;
pub const GROUP_EXECUTE: u32 = 0b100000;

pub const OTHER_READ: u32 = 0b1000000;
pub const OTHER_WRITE: u32 = 0b10000000;
pub const OTHER_EXECUTE: u32 = 0b100000000;

impl NyPermsT {
    pub fn fmt_perms(perms: &Self) -> String {
        let mut out: [char; 10] = ['-'; 10];

        out[1] = if (perms.bits & USER_READ) != 0 {
            'r'
        } else {
            '-'
        };
        out[2] = if (perms.bits & USER_WRITE) != 0 {
            'w'
        } else {
            '-'
        };
        out[3] = if (perms.bits & USER_EXECUTE) != 0 {
            'x'
        } else {
            '-'
        };

        out[4] = if (perms.bits & GROUP_READ) != 0 {
            'r'
        } else {
            '-'
        };
        out[5] = if (perms.bits & GROUP_WRITE) != 0 {
            'w'
        } else {
            '-'
        };
        out[6] = if (perms.bits & GROUP_EXECUTE) != 0 {
            'x'
        } else {
            '-'
        };

        out[7] = if (perms.bits & OTHER_READ) != 0 {
            'r'
        } else {
            '-'
        };
        out[8] = if (perms.bits & OTHER_WRITE) != 0 {
            'w'
        } else {
            '-'
        };
        out[9] = if (perms.bits & OTHER_EXECUTE) != 0 {
            'x'
        } else {
            '-'
        };
        out.iter().collect()
    }

    pub fn chmod(&mut self, input: String) {
        let allowed_chars: [char; 8] = ['u', 'g', 'o', '+', '-', 'r', 'w', 'x'];

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
