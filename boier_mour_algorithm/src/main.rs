use rand::Rng;
use std::time::Instant;

// Наивный алгоритм полного перебора
fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let m = pattern.len();
    let n = text.len();

    if m == 0 || m > n {
        return result;
    }

    for i in 0..=n - m {
        let mut j = 0;
        while j < m && text.as_bytes()[i + j] == pattern.as_bytes()[j] {
            j += 1;
        }
        if j == m {
            result.push(i);
        }
    }
    result
}

// Оптимизация с префиксными сдвигами (KMP)
fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let m = pattern.len();
    let n = text.len();

    if m == 0 || m > n {
        return result;
    }

    let lps = compute_lps(pattern);
    let mut i = 0;
    let mut j = 0;

    while i < n {
        if pattern.as_bytes()[j] == text.as_bytes()[i] {
            i += 1;
            j += 1;
        }
        if j == m {
            result.push(i - j);
            j = lps[j - 1];
        } else if i < n && pattern.as_bytes()[j] != text.as_bytes()[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    result
}

fn compute_lps(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pattern.as_bytes()[i] == pattern.as_bytes()[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}

// Оптимизация с суффиксными сдвигами (Boyer-Moore-Horspool)
fn horspool_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let m = pattern.len();
    let n = text.len();

    if m == 0 || m > n {
        return result;
    }

    let mut bad_char = [m; 256];
    let pattern_bytes = pattern.as_bytes();

    for i in 0..m - 1 {
        bad_char[pattern_bytes[i] as usize] = m - 1 - i;
    }

    let mut i = 0;
    while i <= n - m {
        let mut j = m - 1;
        while j > 0 && text.as_bytes()[i + j] == pattern_bytes[j] {
            j -= 1;
        }
        if j == 0 && text.as_bytes()[i] == pattern_bytes[0] {
            result.push(i);
        }
        i += bad_char[text.as_bytes()[i + m - 1] as usize];
    }
    result
}

// Алгоритм Бойера-Мура
fn boyer_moore_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let m = pattern.len();
    let n = text.len();

    if m == 0 || m > n {
        return result;
    }

    let (bad_char, good_suffix) = preprocess(pattern);
    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    let mut i = 0;
    while i <= n - m {
        let mut j = m - 1;
        while j != usize::MAX && text_bytes[i + j] == pattern_bytes[j] {
            j = j.wrapping_sub(1);
        }
        if j == usize::MAX {
            result.push(i);
            i += good_suffix[0];
        } else {
            let bc_shift = bad_char[text_bytes[i + j] as usize].saturating_sub(m - 1 - j);
            let gs_shift = good_suffix[j];
            i += bc_shift.max(gs_shift);
        }
    }
    result
}

fn preprocess(pattern: &str) -> ([usize; 256], Vec<usize>) {
    let m = pattern.len();
    let pattern_bytes = pattern.as_bytes();

    // Таблица плохих символов
    let mut bad_char = [m; 256];
    for (i, &c) in pattern_bytes.iter().enumerate() {
        bad_char[c as usize] = i;
    }

    // Таблица хороших суффиксов
    let mut borders = vec![0; m + 1];
    let mut shifts = vec![m; m + 1];
    let mut f = 0;

    for i in (0..m).rev() {
        while f > 0 && pattern_bytes[i] != pattern_bytes[m - 1 - f] {
            f = borders[f];
        }
        if pattern_bytes[i] == pattern_bytes[m - 1 - f] {
            f += 1;
        }
        borders[m - i] = f;
    }

    let mut j = borders[0];
    for i in 0..=m {
        if j == 0 || shifts[i] == m - j {
            shifts[i] = j;
        } else {
            shifts[i] = shifts[i].min(m - j);
        }
        if i == j {
            j = borders[j];
        }
    }

    (bad_char, shifts)
}

fn main() {
    let mut rng = rand::rng();
    let test_cases = vec![
        (
            "random",
            (0..10)
                .map(|_| rng.random_range('a'..'z') as char)
                .collect::<String>(),
        ),
        ("repeated", "abc".repeat(5)),
        ("worst_case", "a".repeat(10) + "b"),
        ("best_case", "a".repeat(10)),
    ];

    let patterns = vec![
        ("repeated", "abc"),
        ("worst_case", "ab"),
        ("best_case", "a"),
    ];

    println!("| Тестовый случай | Алгоритм           | Время выполнения |");
    println!("|-----------------|--------------------|------------------|");

    for (case_name, text) in test_cases {
        for (_, pattern) in &patterns {
            if text.len() < pattern.len() {
                continue;
            }

            let text_str = &text;
            let pattern_str = pattern;

            let start = Instant::now();
            naive_search(text_str, pattern_str);
            let naive_time = start.elapsed();

            let start = Instant::now();
            kmp_search(text_str, pattern_str);
            let kmp_time = start.elapsed();

            let start = Instant::now();
            horspool_search(text_str, pattern_str);
            let horspool_time = start.elapsed();

            let start = Instant::now();
            boyer_moore_search(text_str, pattern_str);
            let bm_time = start.elapsed();

            println!("| {:<15} | {:<18} | {:?} |", case_name, "Naive", naive_time);
            println!("| {:<15} | {:<18} | {:?} |", case_name, "KMP", kmp_time);
            println!(
                "| {:<15} | {:<18} | {:?} |",
                case_name, "Horspool", horspool_time
            );
            println!(
                "| {:<15} | {:<18} | {:?} |",
                case_name, "Boyer-Moore", bm_time
            );
        }
    }
}
