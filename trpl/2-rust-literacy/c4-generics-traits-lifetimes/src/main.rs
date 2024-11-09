use structs::*;
use std::cmp::PartialOrd;

mod structs;

fn main() {
    c4_2();
}

fn c4_3() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably know, people".to_string(),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summary());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship".to_string(),
        location: "Pittsburg, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburg Penguins once again are the best hockey team in the NHL.".to_string(),
    };

    notify(tweet);
    notify(article);
}

#[allow(dead_code)]
fn c4_1() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {result}");
    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6_000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);

    println!("The largest number is {result}");
    assert_eq!(result, 6_000);
}

#[allow(dead_code)]
fn c4_2() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {result}");
    assert_eq!(result, 100);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);

    println!("The largest number is {result}");
    assert_eq!(result, 'y');
}

fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
