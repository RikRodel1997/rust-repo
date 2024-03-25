use std::collections::HashMap;


fn main() {
    // vectors();
    // strings();   
    hash_maps();
}


#[allow(dead_code)]
fn hash_maps() {
    let mut scores_insert = HashMap::new();
    scores_insert.insert("Blue".to_string(), 10);
    scores_insert.insert("Red".to_string(), 15);
    println!("{:?}", scores_insert);
    let teams = vec![
        "Blue".to_string(), "Red".to_string()
    ];
    let initial_scores = vec![10, 15];
    let scores_zip: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores_zip);

    for (key, value) in &scores_insert {
        let value_in_scores_zip = *(scores_zip.get(key).unwrap());
        assert_eq!(value, value_in_scores_zip);
    }

}

#[allow(dead_code)]
fn strings() {
    let data = "initial contents";
    let mut s = data.to_string();
    s.push_str(", added contents");
    println!("{}", s);
    for b in s.bytes() {
        println!("{}", b);
    }
}

#[allow(dead_code)]
fn vectors() {
    // let v = vec![1, 2, 3];
    let mut v: Vec<i32>= Vec::new();
    v.push(5);
    v.push(v[0] + 1);
    v.push(v[1] + 1);
    v.push(v[2] + 1);

    let fourth = &v[3];
    assert_eq!(fourth, &(v[2] + 1));
    let fourth_get = v.get(3).unwrap();
    assert_eq!(fourth, fourth_get);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}