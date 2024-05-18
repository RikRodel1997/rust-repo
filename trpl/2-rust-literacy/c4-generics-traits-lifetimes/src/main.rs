fn main() {
    c4_3();
}

fn c4_3() {
    
}

#[allow(dead_code)]
fn c4_1() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("The largest number is {result}");
    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6_000, 89, 54, 2, 43, 8];
    let result = largest_i32(&numbers);

    println!("The largest number is {result}");
    assert_eq!(result, 6_000);
}

#[allow(dead_code)]
fn c4_2() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("The largest number is {result}");
    assert_eq!(result, 100);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);

    println!("The largest number is {result}");
    assert_eq!(result, 'y');
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// fn largest_t<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }