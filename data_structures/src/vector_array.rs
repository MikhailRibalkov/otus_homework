#[derive(Debug)]
pub struct MyVec<T: Copy> {
    data: Box<[Option<T>]>,
    length: usize,
    capacity: usize,
}

impl<T: Copy> MyVec<T> {
    // Создание нового вектора с заданной вместимостью
    pub fn with_capacity(capacity: usize) -> Self {
        let data = vec![None; capacity].into_boxed_slice();
        MyVec {
            data,
            length: 0,
            capacity,
        }
    }

    // Добавление элемента в вектор
    pub fn push(&mut self, value: T) {
        if self.length == self.capacity {
            self.resize();
        }
        self.data[self.length] = Some(value);
        self.length += 1;
    }

    // Получение элемента по индексу
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    // Удаление элемента по индексу
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let value = self.data[index].take();
            self.length -= 1;
            // Сдвиг элементов влево
            for i in index..self.length {
                self.data[i] = self.data[i + 1].take();
            }
            self.data[self.length] = None; // Удаляем последний элемент
            value
        } else {
            None
        }
    }

    // Получение длины вектора
    pub fn len(&self) -> usize {
        self.length
    }

    // Проверка, пустой ли вектор
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Изменение размера вектора
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_data = vec![None; new_capacity].into_boxed_slice();
        new_data[..self.length].copy_from_slice(&self.data[..self.length]);
        self.data = new_data;
        self.capacity = new_capacity;
    }
}

fn main() {}
