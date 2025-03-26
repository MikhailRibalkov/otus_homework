pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn _state(&self, room_name: String, device_name: String) -> String;
    fn create_report(&self) -> String;
}
