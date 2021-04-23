use log::error;

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub fn register_from_string(s: &str, reg_array: &mut [REGISTER]) {
    let key_val_pairs: Vec<&str> = s.split("\n").collect();
    for key_val_pair in key_val_pairs {
        if key_val_pair.is_empty() {
            continue;
        }
        let sep: Vec<&str> = key_val_pair.split(":").collect();
        let key = sep[0].parse::<usize>().unwrap();
        let val = sep[1].parse::<i32>().unwrap();
        let locked = sep[2];
        reg_array[key].content = val;
        if locked == "1" {
            reg_array[key].locked = true;
        } else {
            reg_array[key].locked = false;
        }
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

    #[test]
    fn test_register_from_string() {
        let s = "0:5:1\n1:10:0";
        let mut m = [REGISTER{ content: 0, locked: false }; 2];
        register_from_string(s, &mut m);
        assert_eq!(m[0], REGISTER{ content: 5, locked: true });
    }
}
