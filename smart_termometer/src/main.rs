struct _SmartTermometer {
    name: String,
    temperature: f32,
}

impl _SmartTermometer {
    fn _new() -> _SmartTermometer {
        _SmartTermometer {
            name: "Smart Termometer".to_string(),
            temperature: 0.0,
        }
    }

    fn _get_temperature(&self) -> f32 {
        self.temperature
    }

    fn _set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {}
