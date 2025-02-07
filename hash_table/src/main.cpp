#include <iostream>
#include <list>
#include <string>
#include <functional> // std::hash

// Константа для размера хеш-таблицы.  Можно менять, но должна быть степенью двойки для оптимизации.
const int CAPACITY = 16;

// Структура для хранения пар ключ-значение в хеш-таблице.
template <typename K, typename V>
struct Entry {
    K key;
    V value;
};

// Реализация хеш-таблицы с использованием метода цепочек.
template <typename K, typename V>
class ChainedHashTable {
private:
    std::list<Entry<K, V>> table[CAPACITY]; // Массив связанных списков (buckets).
    int size; // Количество элементов в хеш-таблице.

    // Хеш-функция.  Возвращает индекс в таблице на основе ключа.
    size_t hash(const K& key) const {
        return std::hash<K>{}(key) & (CAPACITY - 1); // std::hash и битовая маска
    }

public:
    // Создает новую хеш-таблицу с указанной вместимостью.
    ChainedHashTable() : size(0) {}

    // Вставляет пару ключ-значение в хеш-таблицу.
    void insert(const K& key, const V& value) {
        size_t index = hash(key);
        std::list<Entry<K, V>>& bucket = table[index];

        // Проверяем, существует ли ключ уже в списке. Если да, заменяем значение.
        for (auto& entry : bucket) {
            if (entry.key == key) {
                entry.value = value;
                return;
            }
        }

        // Ключ не существует, добавляем новую запись в список.
        bucket.push_front({key, value});
        size++;
    }

    // Получает значение, связанное с ключом. Возвращает nullptr, если ключ не найден.
    V* get(const K& key) {
        size_t index = hash(key);
        std::list<Entry<K, V>>& bucket = table[index];

        for (auto& entry : bucket) {
            if (entry.key == key) {
                return &entry.value; // Возвращаем указатель на значение.
            }
        }

        return nullptr; // Ключ не найден.
    }

    // Удаляет пару ключ-значение из хеш-таблицы.
    bool remove(const K& key) {
        size_t index = hash(key);
        std::list<Entry<K, V>>& bucket = table[index];

        for (auto it = bucket.begin(); it != bucket.end(); ++it) {
            if (it->key == key) {
                bucket.erase(it);
                size--;
                return true; // Элемент успешно удален.
            }
        }

        return false; // Ключ не найден.
    }

    // Возвращает текущее количество элементов в хеш-таблице.
    int len() const {
        return size;
    }

    // Возвращает true, если хеш-таблица пуста.
    bool is_empty() const {
        return size == 0;
    }

    // Очищает хеш-таблицу.
    void clear() {
        for (int i = 0; i < CAPACITY; ++i) {
            table[i].clear();
        }
        size = 0;
    }
};

int main() {
    ChainedHashTable<std::string, int> ht;

    ht.insert("apple", 1);
    ht.insert("banana", 2);
    ht.insert("cherry", 3);

    int* apple_value = ht.get("apple");
    if (apple_value != nullptr) {
        std::cout << "Value for apple: " << *apple_value << std::endl;
    } else {
        std::cout << "Key 'apple' not found." << std::endl;
    }

    std::cout << "Size of hash table: " << ht.len() << std::endl;

    ht.remove("banana");

    int* banana_value = ht.get("banana");
    if (banana_value != nullptr) {
        std::cout << "Value for banana: " << *banana_value << std::endl;
    } else {
        std::cout << "Key 'banana' not found." << std::endl;
    }

    std::cout << "Size of hash table: " << ht.len() << std::endl;

    ht.clear();
    std::cout << "Is hash table empty? " << ht.is_empty() << std::endl;

    return 0;
}
