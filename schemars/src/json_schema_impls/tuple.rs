use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::{JsonSchema, Result};

macro_rules! tuple_impls {
    ($($len:expr => ($($name:ident)+))+) => {
        $(
            impl<$($name: JsonSchema),+> JsonSchema for ($($name,)+) {
                no_ref_schema!();

                fn schema_name() -> String {
                    ["Tuple_Of".to_owned()$(, $name::schema_name())+].join("_And_")
                }

                fn json_schema(gen: &mut SchemaGenerator) -> Result {
                    let items = vec![
                        $(gen.subschema_for::<$name>()?),+
                    ];
                    Ok(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: ArrayValidation {
                            items: Some(items.into()),
                            max_items: Some($len),
                            min_items: Some($len),
                            ..Default::default()
                        },
                        ..Default::default()
                    }.into())
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (T0)
    2 => (T0 T1)
    3 => (T0 T1 T2)
    4 => (T0 T1 T2 T3)
    5 => (T0 T1 T2 T3 T4)
    6 => (T0 T1 T2 T3 T4 T5)
    7 => (T0 T1 T2 T3 T4 T5 T6)
    8 => (T0 T1 T2 T3 T4 T5 T6 T7)
    9 => (T0 T1 T2 T3 T4 T5 T6 T7 T8)
    10 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9)
    11 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10)
    12 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11)
    13 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12)
    14 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13)
    15 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14)
    16 => (T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{schema_for, schema_object_for};
    use pretty_assertions::assert_eq;

    #[test]
    fn schema_for_map_any_value() {
        let schema = schema_object_for::<(i32, bool)>();
        assert_eq!(
            schema.instance_type,
            Some(SingleOrVec::from(InstanceType::Array))
        );
        assert_eq!(
            schema.array.items,
            Some(SingleOrVec::Vec(vec![
                schema_for::<i32>(),
                schema_for::<bool>()
            ]))
        );
        assert_eq!(schema.array.max_items, Some(2));
        assert_eq!(schema.array.min_items, Some(2));
    }
}