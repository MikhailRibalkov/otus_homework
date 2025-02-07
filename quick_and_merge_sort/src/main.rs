use rand::rng;
use rand::Rng;

fn main() {
    let mut count = 100;

    for _ in 0..5 {
        let now = std::time::Instant::now();
        let mut arr: Vec<i32> = (0..count).map(|_| rng().random_range(0..100)).collect();
        merge_sort(&mut arr);
        println!("count: {} - {:?}", count, now.elapsed().as_millis());
        count *= 10;
    }
}

fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Массив из 0 или 1 элемента уже отсортирован
    }
    quick_sort_recursive(arr, 0, arr.len() - 1);
}

fn quick_sort_recursive<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        // Находим индекс разделения, arr[pivot] находится теперь на своем месте
        let pivot_index = partition(arr, low, high);

        // Рекурсивно сортируем элементы до и после индекса разделения
        if pivot_index > 0 {
            // To avoid underflow when pivot_index is 0
            quick_sort_recursive(arr, low, pivot_index - 1);
        }
        quick_sort_recursive(arr, pivot_index + 1, high);
    }
}

fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
    // Выбираем последний элемент в качестве опорного
    let pivot = arr[high].clone();

    // Индекс меньшего элемента и индикатор правильной позиции для
    // элемента, меньшего, чем pivot
    let mut i = low;

    for j in low..high {
        // Если текущий элемент меньше или равен pivot
        if &arr[j] <= &pivot {
            // Обмениваем arr[i] и arr[j]
            &arr.swap(i, j);
            i += 1;
        }
    }

    // Обмениваем arr[i] с arr[high] (или pivot)
    arr.swap(i, high);

    i // Возвращаем индекс разделения
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Массив из 0 или 1 элемента уже отсортирован
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec(); // Создаем копию левой части
    let mut right = arr[mid..].to_vec(); // Создаем копию правой части

    merge_sort(&mut left); // Рекурсивно сортируем левую часть
    merge_sort(&mut right); // Рекурсивно сортируем правую часть

    // Сливаем отсортированные левую и правую части обратно в исходный массив
    merge(&mut left, &mut right, arr);
}

fn merge<T: Ord + Copy>(left: &mut [T], right: &mut [T], arr: &mut [T]) {
    let mut i = 0; // Индекс для левого массива
    let mut j = 0; // Индекс для правого массива
    let mut k = 0; // Индекс для результирующего массива

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Копируем оставшиеся элементы из левого массива (если есть)
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    // Копируем оставшиеся элементы из правого массива (если есть)
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 1, 4, 2, 8];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8]);
    }

    #[test]
    fn test_quick_sort_with_duplicates() {
        let mut arr = [5, 1, 4, 2, 8, 5, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 4, 5, 5, 8]);
    }

    #[test]
    fn test_quick_sort_mixed_numbers() {
        let mut arr = [-5, 1, -4, 2, -8, 0, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [-8, -5, -4, 0, 1, 2, 5]);
    }

    #[test]
    fn test_quick_sort_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        quick_sort(&mut arr);
        let expected: Vec<i32> = (0..1000).collect();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 1, 4, 2, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8]);
    }

    #[test]
    fn test_merge_sort_with_duplicates() {
        let mut arr = [5, 1, 4, 2, 8, 5, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 4, 5, 5, 8]);
    }

    #[test]
    fn test_merge_sort_mixed_numbers() {
        let mut arr = [-5, 1, -4, 2, -8, 0, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, [-8, -5, -4, 0, 1, 2, 5]);
    }

    #[test]
    fn test_merge_sort_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        merge_sort(&mut arr);
        let expected: Vec<i32> = (0..1000).collect();
        assert_eq!(arr, expected);
    }
}
