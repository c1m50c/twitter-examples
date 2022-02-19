fn main() {
    let option = None.unwrap_or_else(|| {
        3.0 + 0.14
    });

    assert_eq!(option, 3.14);

    let result = "13.37".parse::<f64>()
        .unwrap_or_else(|err| {
            panic!("{:?}: Cannot parse number into a floating-point number.", err)
        });
    
    assert_eq!(result, 13.37);
}