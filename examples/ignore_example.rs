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
    println!("=== 类型转换示例 ===");

    let val = Integer(123);
    let result = match_any!(val,
        Integer(i) => i as f64,
        Float(f) => f,
        _ => 0.0
    );

    println!("数值结果: {}", result);

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

// 辅助 trait 用于类型转换
trait ToFloat {
    fn unwrap_or(self, default: f64) -> f64;
}

impl ToFloat for i32 {
    fn unwrap_or(self, _default: f64) -> f64 {
        self as f64
    }
}

impl ToFloat for f64 {
    fn unwrap_or(self, _default: f64) -> f64 {
        self
    }
}
