struct _SmartSocket {
    name: String,
    description: String,
    is_on: bool,
    power: f32,
}

impl _SmartSocket {
    fn _new() -> _SmartSocket {
        _SmartSocket {
            name: "SmartSocket".to_string(),
            description: "".to_string(),
            is_on: false,
            power: 0.0,
        }
    }

    fn _switch(&mut self) {
        self.is_on = !self.is_on;
    }

    fn _get_power(&self) -> f32 {
        self.power
    }

    fn _get_description(&self) -> String {
        self.description.clone()
    }

    fn _set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {}
