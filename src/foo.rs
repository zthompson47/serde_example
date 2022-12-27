use serde::{de::Visitor, ser::SerializeStruct};

use crate::Bar;

#[derive(Debug, PartialEq)]
pub struct Foo {
    pub a: u32,
    pub b: Vec<u64>,
    pub c: Bar,
}

impl serde::Serialize for Foo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Foo", 3)?;
        state.serialize_field("a", &self.a)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("c", &self.c)?;
        state.end()
    }
}

enum FooField {
    A,
    B,
    C,
}

struct FooFieldVisitor;

impl<'de> Visitor<'de> for FooFieldVisitor {
    type Value = FooField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "a" => Ok(FooField::A),
            "b" => Ok(FooField::B),
            "c" => Ok(FooField::C),
            unknown => Err(serde::de::Error::unknown_field(unknown, &["a", "b", "c"])),
        }
    }
}

impl<'de> serde::Deserialize<'de> for FooField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FooFieldVisitor)
    }
}

struct FooVisitor;

impl<'de> Visitor<'de> for FooVisitor {
    type Value = Foo;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Foo")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut a: Option<u32> = None;
        let mut b: Option<Vec<u64>> = None;
        let mut c: Option<Bar> = None;

        while let Some(key) = map.next_key::<FooField>()? {
            match key {
                FooField::A => {
                    if a.is_some() {
                        return Err(serde::de::Error::duplicate_field("a"));
                    }
                    a = Some(map.next_value::<u32>()?);
                }
                FooField::B => {
                    if b.is_some() {
                        return Err(serde::de::Error::duplicate_field("b"));
                    }
                    b = Some(map.next_value::<Vec<u64>>()?);
                }
                FooField::C => {
                    if c.is_some() {
                        return Err(serde::de::Error::duplicate_field("c"));
                    }
                    c = Some(map.next_value::<Bar>()?);
                }
            }
        }

        let a = a.ok_or(serde::de::Error::missing_field("a"))?;
        let b = b.ok_or(serde::de::Error::missing_field("b"))?;
        let c = c.ok_or(serde::de::Error::missing_field("c"))?;

        Ok(Foo { a, b, c })
    }
}

impl<'de> serde::Deserialize<'de> for Foo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Foo", &["a", "b", "c"], FooVisitor)
    }
}

#[test]
fn test_foo() {
    let f = Foo {
        a: 47,
        b: vec![1, 22, 333],
        c: Bar {
            a: 42,
            b: "qwerty".into(),
        },
    };

    let f_ser = crate::to_string(&f).unwrap();
    assert_eq!(
        f_ser,
        r#"{"a":47,"b":[1,22,333],"c":{"a":42,"b":"qwerty"}}"#
    );

    let f_de = crate::from_str(&f_ser).unwrap();
    assert_eq!(f, f_de);
}
