use match_any::match_any;

#[derive(Debug, PartialEq)]
enum Value {
    A(i32),
    B(String),
    C(f64),
}

#[test]
fn test_backward_compatibility() {
    use Value::*;

    // 确保原有的语法仍然工作
    let val = Value::A(123);
    let result = match_any!(val,
        A(x) => x as f64,
        B(s) => s.len() as f64,
        C(f) => f
    );
    assert_eq!(result, 123.0);

    // 测试带默认分支的匹配
    let val = Value::A(10);
    let result = match_any!(val,
        A(x) => x * 2,
        _ => 0
    );
    assert_eq!(result, 20);
}

#[test]
fn test_cfg_attribute() {
    use Value::*;

    // 测试 #[cfg] 属性（这些分支在没有对应 feature 时会被编译器忽略）
    let val = Value::A(42);
    let result = match_any!(val,
        A(x) => x as f64,
        #[cfg(feature = "never-enabled")]
        B(s) => s.len() as f64,
        #[cfg(feature = "also-never-enabled")]
        C(f) => f * 2.0,
        _ => 999.0
    );
    assert_eq!(result, 42.0);

    // 测试当匹配值需要通过 cfg 控制的分支时
    let val = Value::B("test".to_string());
    let result = match_any!(val,
        A(x) => x as f64,
        #[cfg(feature = "never-enabled")]
        B(s) => s.len() as f64, // 这个分支被 cfg 禁用
        #[cfg(feature = "also-never-enabled")]
        C(f) => f,
        _ => 555.0 // 应该匹配这个分支
    );
    assert_eq!(result, 555.0);
}

#[test]
fn test_complex_patterns() {
    use Value::*;

    // 测试复杂模式匹配
    let values = vec![
        Value::A(1),
        Value::B("hello".to_string()),
        Value::C(std::f64::consts::PI),
    ];

    let results: Vec<String> = values
        .into_iter()
        .map(|val| {
            match_any!(val,
                A(x) => format!("整数: {}", x),
                B(s) => format!("字符串: {}", s),
                C(f) => format!("浮点数: {:.2}", f)
            )
        })
        .collect();

    assert_eq!(results[0], "整数: 1");
    assert_eq!(results[1], "字符串: hello");
    assert_eq!(results[2], "浮点数: 3.14");
}
