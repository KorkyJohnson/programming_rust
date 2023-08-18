fn main() {
    // let v: Vec<i32> = Vec::new();

    let mut v= vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("2nd time the third element is {third}"),
        None => println!("There is no thrid element")
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![44, 55, 66];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // you can vector w/ enums
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("blue".to_string()),
        SpreadSheetCell::Float(4.89),
    ];

}
