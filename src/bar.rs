use serde::{de::Visitor, ser::SerializeStruct};

#[derive(Debug, PartialEq)]
pub struct Bar {
    pub a: u64,
    pub b: String,
}

impl serde::Serialize for Bar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Bar", 2)?;
        state.serialize_field("a", &self.a)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

enum BarField {
    A,
    B,
}

struct BarFieldVisitor;

impl<'de> Visitor<'de> for BarFieldVisitor {
    type Value = BarField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("field identifier")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            "a" => Ok(BarField::A),
            "b" => Ok(BarField::B),
            unknown => Err(serde::de::Error::unknown_field(unknown, &["a", "b"])),
        }
    }
}

impl<'de> serde::Deserialize<'de> for BarField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BarFieldVisitor)
    }
}

struct BarVisitor;

impl<'de> Visitor<'de> for BarVisitor {
    type Value = Bar;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Bar")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut a: Option<u64> = None;
        let mut b: Option<String> = None;

        while let Some(key) = map.next_key::<BarField>()? {
            match key {
                BarField::A => {
                    if a.is_some() {
                        return Err(serde::de::Error::duplicate_field("a"));
                    }
                    a = Some(map.next_value::<u64>()?);
                }
                BarField::B => {
                    if b.is_some() {
                        return Err(serde::de::Error::duplicate_field("b"));
                    }
                    b = Some(map.next_value::<String>()?);
                }
            }
        }

        let a = a.ok_or(serde::de::Error::missing_field("a"))?;
        let b = b.ok_or(serde::de::Error::missing_field("b"))?;

        Ok(Bar { a, b })
    }
}

impl<'de> serde::Deserialize<'de> for Bar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Bar", &["a", "b"], BarVisitor)
    }
}
