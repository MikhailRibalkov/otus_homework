use std::fmt;

#[derive(PartialEq, Default)]
pub struct SmartSocket {
    pub name: String,
    pub is_on: bool,
    pub power: f32,
}

impl SmartSocket {
    pub fn new() -> SmartSocket {
        SmartSocket {
            name: "SmartSocket".to_string(),
            is_on: false,
            power: 0.0,
        }
    }

    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
    }
}

impl fmt::Debug for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.name, self.power, self.is_on)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_smart_socket() {
        let sm: SmartSocket = SmartSocket::new();
        assert_eq!(
            sm,
            SmartSocket {
                name: "SmartSocket".to_string(),
                is_on: false,
                power: 0.0,
            }
        );
    }

    #[test]
    fn test_get_power() {
        let sm: SmartSocket = Default::default();
        assert_eq!(sm.power, 0.0);
    }

    #[test]
    fn test_switch() {
        let mut sm: SmartSocket = Default::default();
        sm.switch();
        assert!(sm.is_on);
    }
}
