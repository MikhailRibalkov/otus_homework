fn king_moves(pos: u8) -> u64 {
    const NOT_A_FILE: u64 = 0xfefefefefefefefe;
    const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

    let mut moves = 0;
    let pos = 1u64 << pos;

    // Варианты возможных ходов
    moves |= pos << 8; // вверх
    moves |= pos >> 8; // вниз
    moves |= (pos << 1) & NOT_A_FILE; // вправо
    moves |= (pos >> 1) & NOT_H_FILE; // влево
    moves |= (pos << 9) & NOT_A_FILE; // вверх-вправо
    moves |= (pos << 7) & NOT_H_FILE; // вверх-влево
    moves |= (pos >> 7) & NOT_A_FILE; // вниз-вправо
    moves |= (pos >> 9) & NOT_H_FILE; // вниз-влево

    moves
}

fn horse_moves(pos: u8) -> u64 {
    let mut moves = 0;
    let pos = 1u64 << pos;
    const NOT_A_FILE: u64 = 0xfefefefefefefefe;
    const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
    const NOT_AB_FILE: u64 = 0xfcfcfcfcfcfcfcfc;
    const NOT_GH_FILE: u64 = 0x3f3f3f3f3f3f3f3f;

    moves |= (pos << 17) & NOT_A_FILE; // вверх-влево
    moves |= (pos << 15) & NOT_H_FILE; // вверх-вправо
    moves |= (pos >> 17) & NOT_H_FILE; // вниз-вправо
    moves |= (pos >> 15) & NOT_A_FILE; // вниз-влево
    moves |= (pos << 10) & NOT_AB_FILE; // влево-вверх
    moves |= (pos << 6) & NOT_GH_FILE; // вправо-вверх
    moves |= (pos >> 10) & NOT_GH_FILE; // вправо-вниз
    moves |= (pos >> 6) & NOT_AB_FILE; // влево-вниз

    moves
}

fn count_ones_v1(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

fn count_ones_kernighan(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        n &= n - 1; // Убираем младший установленный бит
        count += 1;
    }
    count
}

fn main() {
    let moves = king_moves(0);
    let move_count = moves.count_ones();
    println!("king:\n{}\n{}", move_count, moves);
    let moves = horse_moves(0);
    let move_count = moves.count_ones();
    println!("horse:\n{}\n{}", move_count, moves);
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;
    #[test]
    fn test_horse_moves() {
        for i in 0..10 {
            let input_filename = format!("Bitboard_Horse/test.{}.in", i);
            let output_filename = format!("Bitboard_Horse/test.{}.out", i);

            let mut input_file =
                File::open(&input_filename).expect("Не удалось открыть файл ввода");
            let mut input_data = String::new();
            input_file
                .read_to_string(&mut input_data)
                .expect("Ошибка чтения файла ввода");

            let position: u8 = input_data
                .trim()
                .parse()
                .expect("Ошибка парсинга входных данных");
            let moves = horse_moves(position);
            let move_count = moves.count_ones();

            let mut output_file =
                File::open(&output_filename).expect("Не удалось открыть файл вывода");
            let mut expected_output = String::new();
            output_file
                .read_to_string(&mut expected_output)
                .expect("Ошибка чтения файла вывода");
            let mut expected_lines = expected_output.lines();
            let expected_move_count: u32 = expected_lines
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Ошибка парсинга количества ходов");
            let expected_moves: u64 = expected_lines
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Ошибка парсинга битовой маски");

            assert_eq!(
                move_count, expected_move_count,
                "Тест {} не пройден (количество ходов)",
                i
            );
            assert_eq!(
                moves, expected_moves,
                "Тест {} не пройден (битовая маска ходов)",
                i
            );
        }
    }

    #[test]
    fn test_king_moves() {
        for i in 0..10 {
            let input_filename = format!("Bitboard_King/test.{}.in", i);
            let output_filename = format!("Bitboard_King/test.{}.out", i);

            let mut input_file =
                File::open(&input_filename).expect("Не удалось открыть файл ввода");
            let mut input_data = String::new();
            input_file
                .read_to_string(&mut input_data)
                .expect("Ошибка чтения файла ввода");

            let position: u8 = input_data
                .trim()
                .parse()
                .expect("Ошибка парсинга входных данных");
            let moves = king_moves(position);
            let move_count = moves.count_ones();

            let mut output_file =
                File::open(&output_filename).expect("Не удалось открыть файл вывода");
            let mut expected_output = String::new();
            output_file
                .read_to_string(&mut expected_output)
                .expect("Ошибка чтения файла вывода");
            let mut expected_lines = expected_output.lines();
            let expected_move_count: u32 = expected_lines
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Ошибка парсинга количества ходов");
            let expected_moves: u64 = expected_lines
                .next()
                .unwrap()
                .trim()
                .parse()
                .expect("Ошибка парсинга битовой маски");

            assert_eq!(
                move_count, expected_move_count,
                "Тест {} не пройден (количество ходов)",
                i
            );
            assert_eq!(
                moves, expected_moves,
                "Тест {} не пройден (битовая маска ходов)",
                i
            );
        }
    }
}
