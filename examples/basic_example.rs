use match_any::match_any;

#[derive(Debug, PartialEq)]
enum DatabaseValue {
    Integer(i32),
    Text(String),
    Float(f64),
    Boolean(bool),
}

fn main() {
    use DatabaseValue::*;

    println!("=== 基本模式匹配示例 ===");

    let values = vec![
        Integer(42),
        Text("hello".to_string()),
        Float(3.14),
        Boolean(true),
    ];

    for value in values {
        let result = match_any!(value,
            Integer(i) => format!("整数: {}", i),
            Text(s) => format!("文本: {}", s),
            Float(f) => format!("浮点数: {:.2}", f),
            Boolean(b) => format!("布尔值: {}", b)
        );

        println!("{}", result);
    }

    println!();
    println!("=== 类型匹配示例 ===");

    let numbers = vec![
        Integer(-5),
        Integer(0),
        Integer(10),
        Float(-2.5),
        Float(7.8),
    ];

    for num in numbers {
        let category = match_any!(num,
            Integer(_) => "整数类型",
            Float(_) => "浮点数类型",
            _ => "其他类型"
        );
        println!("{:?} -> {}", num, category);
    }

    println!();
    println!("=== 类型转换示例 ===");

    let val = Integer(123);
    let result = match_any!(val,
        Integer(i) => i as f64,
        Float(f) => f,
        _ => 0.0
    );

    println!("数值结果: {}", result);

    println!();
    println!("=== cfg 条件编译示例 ===");

    let val = Text("test".to_string());
    let result = match_any!(val,
        Integer(i) => format!("整数: {}", i),
        #[cfg(feature = "extended-features")]
        Text(s) => format!("扩展文本处理: {}", s.to_uppercase()),
        Text(s) => format!("基础文本: {}", s),
        _ => "未知类型".to_string()
    );

    println!("处理结果: {}", result);

    println!();
    println!("=== 使用通配符的示例 ===");

    let val = Boolean(false);
    let result = match_any!(val,
        Integer(i) => format!("整数值: {}", i),
        Float(f) => format!("浮点数值: {}", f),
        _ => "其他类型".to_string()
    );

    println!("结果: {}", result);
}
