use smart_home::device::smartsocket::SmartSocket;
use smart_home::device::smarttermometer::SmartThermometer;
use smart_home::infoprovider::deviceinfoprovider::DeviceInfoProvider;
use smart_home::smart_house::smarthouse::SmartHouse;

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn _state(&self, _: String, _: String) -> String {
        String::from("")
    }
    fn create_report(&self) -> String {
        format!(
            "Report for {} is {}, power is {}",
            self.socket.name, self.socket.is_on, self.socket.power
        )
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn _state(&self, _: String, _: String) -> String {
        String::from("")
    }

    fn create_report(&self) -> String {
        format!(
            "Report for {} is {}, power is {}\nfor {}, temperature is {}",
            self.socket.name,
            self.socket.is_on,
            self.socket.power,
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

    //// Инициализация дома
    let house = SmartHouse::new();

    //// Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    //// todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    //// Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    //// todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    //// Выводим отчёты на экран:
    println!("Rooms in the house: {:?}", house.get_rooms());
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
