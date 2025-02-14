use std::collections::HashMap;
// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

struct Room {
    devices: Vec<String>,
}

struct SmartHouse {
    /* todo: данные умного дома */
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    fn _get_rooms(&self) -> Vec<String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.rooms.keys().cloned().collect()
    }

    fn _devices(&self, room: &str) -> Vec<String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.rooms.get(room).unwrap().devices.clone()
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        device_provider: &dyn DeviceInfoProvider,
    ) -> String {
        device_provider.create_report()
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn state(&self, room_name: String, device_name: String) -> String;
    fn create_report(&self) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket {
    name: String,
    is_on: bool,
    power: f32,
}

impl SmartSocket {
    fn new() -> SmartSocket {
        SmartSocket {
            name: "SmartSocket".to_string(),
            is_on: false,
            power: 0.0,
        }
    }

    fn switch(&mut self) {
        self.is_on = !self.is_on;
    }

    fn get_power(&self) -> f32 {
        self.power
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

struct SmartThermometer {
    name: String,
    temperature: f32,
}

impl SmartThermometer {
    fn new() -> SmartThermometer {
        SmartThermometer {
            name: "SmartTermometer".to_string(),
            temperature: 0.0,
        }
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn _set_name(&mut self, name: String) {
        self.name = name;
    }
}
// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn state(&self, _: String, _: String) -> String {
        String::from("")
    }
    fn create_report(&self) -> String {
        format!(
            "Report for {} is {}, power is {}",
            self.socket.name,
            self.socket.is_on,
            self.socket.get_power()
        )
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn state(&self, _: String, _: String) -> String {
        String::from("")
    }
    fn create_report(&self) -> String {
        format!(
            "Report for {} is {}, power is {}\nfor {}, temperature is {}",
            self.socket.name,
            self.socket.is_on,
            self.socket.get_power(),
            self.thermo.name,
            self.thermo.temperature
        )
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new();
    let socket2 = SmartSocket::new();
    let thermo = SmartThermometer::new();

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
