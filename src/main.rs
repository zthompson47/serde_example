use testserde::{Bar, Foo};

fn main() {
    let f = Foo {
        a: 47,
        b: vec![1, 22, 333],
        c: Bar {
            a: 42,
            b: "qwerty".into(),
        },
    };

    let f_ser = testserde::to_string(&f).unwrap();
    println!("{f_ser}");
    assert_eq!(f_ser, r#"{"a":47,"b":[1,22,333],"c":{"a":42,"b":"qwerty"}}"#);

    let f_de = testserde::from_str(&f_ser).unwrap();
    println!("{f_de:#?}");
    assert_eq!(f, f_de);
}
