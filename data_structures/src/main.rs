use data_structures::vector_array::MyVec;
fn main() {
    let mut vec = MyVec::with_capacity(2);
    vec.push(1);
    vec.push(2);
    vec.push(3); // Должен вызвать увеличение емкости

    println!("MyVec: {:?}", vec);
    println!("Element at index 1: {:?}", vec.get(1));
    println!("Length: {}", vec.len());
    println!("Is empty: {}", vec.is_empty());

    vec.remove(1);
    println!("After removing element at index 1: {:?}", vec);
}
