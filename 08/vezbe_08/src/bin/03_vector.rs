enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn create_mixed_type_vector() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {i}"),
            SpreadsheetCell::Float(f) => println!("Float: {f}"),
            SpreadsheetCell::Text(s) => println!("Text: {s}"),
        }
    }
}

fn main() {
    create_mixed_type_vector();
}
