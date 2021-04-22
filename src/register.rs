use log::error;

#[derive(Debug, Clone, Copy)]
pub struct REGISTER {
    pub content: i32,
    pub locked: bool,
}

impl REGISTER {
    /// returns the state of the register lock
    pub fn is_locked(self) -> bool {
        return self.locked;
    }

    /// toggles the register lock
    pub fn toggle_lock(&mut self) {
        self.locked = !self.locked;
    }

    /// sets a value to the REGISTER according to the lock and returns if it was successful
    pub fn set(&mut self, val: i32) -> bool {
        if !self.locked {
            self.content = val;
            return true;
        }
        return false;
    }

    /// uses REGISTER.set() under the hood but will crash the VM with a error if the register is locked
    pub fn do_set(&mut self, val: i32) {
        let sucessful = self.set(val);
        if !sucessful {
            error!("unable to set register content due to it being locked (Refer to the perling info log for more info)")
        }
    }
}

pub fn register_from_string(s: &str, reg_array: *[REGISTER]) {
    let key_val_pairs = s.split("\n");
    for key_val_pair in key_val_pairs {
        let sep = key_val_pair.split(":");
        let key = sep[0];
        let val = sep[1];
        reg_array[key] = val;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_is_locked() {
        let mut test_reg = REGISTER {
            content: 0,
            locked: false,
        };
        assert_eq!(test_reg.is_locked(), false);

        test_reg.locked = true;
        assert_eq!(test_reg.is_locked(), true);
    }

    #[test]
    fn test_register_toggle_lock() {
        let mut test_reg = REGISTER {
            content: 0,
            locked: false,
        };
        assert_eq!(test_reg.locked, false);

        test_reg.toggle_lock();
        assert_eq!(test_reg.locked, true);

        test_reg.toggle_lock();
        assert_eq!(test_reg.locked, false);    

        test_reg.toggle_lock();
        assert_eq!(test_reg.locked, true);    
    }

    #[test]
    fn test_register_set() {
        let mut test_reg = REGISTER {
            content: 0,
            locked: false,
        };
        let mut sucessful = test_reg.set(3);
        assert_eq!(test_reg.content, 3);
        assert_eq!(sucessful, true);


        test_reg.locked = true;
        sucessful = test_reg.set(4);
        assert_eq!(test_reg.content, 3);
        assert_eq!(sucessful, false);

    }
}
