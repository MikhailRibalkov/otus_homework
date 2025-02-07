fn main() {
    println!("Hello, world!");
}

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..len {
        let mut min_index = i;

        // Поиск минимального элемента в оставшейся части массива
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // Обмен минимального элемента с текущим
        arr.swap(i, min_index);
    }
}

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // Построение max-heap из входного массива
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Извлечение элементов из кучи по одному и перемещение их в конец массива
    for i in (1..n).rev() {
        // Перемещение текущего корня в конец
        arr.swap(0, i);

        // Вызов heapify для уменьшенной кучи
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i; // Инициализируем largest как корень
    let left = 2 * i + 1; // левый ребенок
    let right = 2 * i + 2; // правый ребенок

    // Если левый ребенок больше корня
    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    // Если правый ребенок больше, чем пока что largest
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // Если largest не корень
    if largest != i {
        // Обмениваем местами корень и largest
        arr.swap(i, largest);

        // Рекурсивно heapify поддерево
        heapify(arr, n, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_array() {
        let mut arr = [3, 1, 4, 5, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_strings() {
        let mut arr = ["banana", "apple", "cherry", "date"];
        selection_sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = [5, 1, 4, 2, 8];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8]);
    }
}
