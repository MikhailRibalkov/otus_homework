use crate::infoprovider::deviceinfoprovider::DeviceInfoProvider;
use crate::smart_house::room::Room;
use std::collections::HashMap;
pub struct SmartHouse {
    /* todo: данные умного дома */
    _rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new() -> Self {
        Self {
            _rooms: HashMap::new(),
        }
    }

    fn _get_rooms(&self) -> Vec<String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self._rooms.keys().cloned().collect()
    }

    fn _devices(&self, room: &str) -> Vec<String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self._rooms.get(room).unwrap().devices.clone()
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        device_provider: &dyn DeviceInfoProvider,
    ) -> String {
        device_provider.create_report()
    }
}
