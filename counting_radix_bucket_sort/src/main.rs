use rand::rng;
use rand::Rng;

fn main() {
    let mut count = 100;

    for _ in 0..5 {
        let now = std::time::Instant::now();
        let mut arr: Vec<f64> = (0..count).map(|_| rng().random_range(0.0..1.0)).collect();
        bucket_sort(&mut arr);
        println!("count: {} - {:?}", count, now.elapsed().as_millis());
        count *= 10;
    }
}

fn counting_sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    // 1. Находим максимальный элемент в массиве
    let max_val = *arr.iter().max().unwrap();

    // 2. Создаем массив для подсчета элементов (counts)
    let mut counts = vec![0; max_val + 1];

    // 3. Заполняем массив counts количеством каждого элемента из входного массива
    for &val in arr.iter() {
        counts[val] += 1;
    }

    // 4. Преобразуем массив counts, чтобы он содержал позиции каждого элемента в отсортированном массиве.
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    // 5. Создаем выходной массив
    let mut output = vec![0; arr.len()];

    // 6. Заполняем выходной массив в отсортированном порядке
    for &val in arr.iter().rev() {
        // Итерируем в обратном порядке для стабильности
        let index = counts[val] - 1;
        output[index] = val;
        counts[val] -= 1;
    }

    // 7. Копируем отсортированные элементы обратно в исходный массив
    arr.copy_from_slice(&output);
}

fn radix_sort(arr: &mut [usize]) {
    if arr.is_empty() {
        return;
    }

    // Находим максимальное значение, чтобы определить количество разрядов
    let max_val = *arr.iter().max().unwrap();
    let num_digits = (max_val as f64).log10() as usize + 1;

    // Сортируем для каждого разряда, начиная с наименее значимого
    for digit in 0..num_digits {
        counting_sort_for_digit(arr, digit);
    }
}

fn counting_sort_for_digit(arr: &mut [usize], digit: usize) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = [0; 10]; // 10 цифр (0-9)

    // Подсчитываем количество вхождений каждой цифры в текущем разряде
    for &num in arr.iter() {
        let digit_val = (num / 10_usize.pow(digit as u32)) % 10;
        count[digit_val] += 1;
    }

    // Накапливаем count, чтобы получить правильные позиции в output
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Строим выходной массив
    for &num in arr.iter().rev() {
        let digit_val = (num / 10_usize.pow(digit as u32)) % 10;
        output[count[digit_val] - 1] = num;
        count[digit_val] -= 1;
    }

    // Копируем отсортированный результат обратно в исходный массив
    arr.copy_from_slice(&output);
}

fn bucket_sort(arr: &mut [f64]) {
    if arr.is_empty() {
        return;
    }

    let n = arr.len();

    // 1. Создаем корзины (buckets)
    let mut buckets: Vec<Vec<f64>> = Vec::with_capacity(n);
    for _ in 0..n {
        buckets.push(Vec::new());
    }

    // 2. Распределяем элементы по корзинам
    for &val in arr.iter() {
        // Предполагаем, что входные значения находятся в диапазоне [0, 1)
        // В противном случае требуется нормализация
        if val < 0.0 || val >= 1.0 {
            panic!("Bucket Sort requires input values to be in the range [0, 1)");
        }

        let bucket_index = (val * n as f64) as usize; // Вычисляем индекс корзины

        buckets[bucket_index].push(val);
    }

    // 3. Сортируем элементы в каждой корзине
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Используем сортировку вставками или другой подходящий алгоритм
    }

    // 4. Собираем элементы из корзин обратно в исходный массив
    let mut k = 0;
    for bucket in &buckets {
        for &val in bucket.iter() {
            arr[k] = val;
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = [5, 1, 4, 2, 8];
        counting_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 8]);
    }

    #[test]
    fn test_radix_sort() {
        let mut arr = [170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, [2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_bucket_sort() {
        let mut arr = [0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.1234, 0.3434, 0.565, 0.656, 0.665, 0.897]);
    }
}
