use simple_sorting::sorting::SortObject;

fn main() {
    let mut num = 2;
    let mut n = 10;
    let mut unsorted_obect = SortObject::new();
    while num > 0 {
        unsorted_obect.get_array(n);
        unsorted_obect.insert_sort();

        num -= 1;
        n *= 2;
    }
}
