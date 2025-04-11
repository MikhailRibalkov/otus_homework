use crate::infoprovider::deviceinfoprovider::DeviceInfoProvider;
use crate::smart_house::room::Room;
use std::collections::HashMap;

#[derive(Default)]
pub struct SmartHouse {
    /* todo: данные умного дома */
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new() -> Self {
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

    pub fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        device_provider: &dyn DeviceInfoProvider,
    ) -> String {
        device_provider.create_report()
    }
}
