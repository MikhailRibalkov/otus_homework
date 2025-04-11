use std::fmt;

#[derive(PartialEq, Default)]
pub struct SmartSocket {
    name: String,
    is_on: bool,
    power: f32,
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

    pub fn get_power(&self) -> f32 {
        self.power
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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
        assert_eq!(sm.get_power(), 0.0);
    }

    #[test]
    fn test_set_name() {
        let mut sm: SmartSocket = Default::default();
        sm.set_name(String::from("test"));
        assert_eq!(sm.name, String::from("test"));
    }

    #[test]
    fn test_switch() {
        let mut sm: SmartSocket = Default::default();
        sm.switch();
        assert!(sm.is_on);
    }
}
