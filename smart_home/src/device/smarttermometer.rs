use std::fmt;

#[derive(Default, PartialEq)]
pub struct SmartThermometer {
    pub name: String,
    pub temperature: f32,
}

impl SmartThermometer {
    pub fn new() -> SmartThermometer {
        SmartThermometer {
            name: "SmartTermometer".to_string(),
            temperature: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl fmt::Debug for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_smatr_termometer() {
        let st = SmartThermometer::new();
        assert_eq!(
            st,
            SmartThermometer {
                name: String::from("SmartTermometer"),
                temperature: 0.0,
            }
        );
    }

    #[test]
    fn test_set_name() {
        let mut st: SmartThermometer = Default::default();
        st.set_name(String::from("test"));
        assert_eq!(st.name, String::from("test"))
    }
}
