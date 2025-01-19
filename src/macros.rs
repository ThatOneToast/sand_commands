#[macro_export]
macro_rules! target_filter {
    // Error case: when no @ type is provided
    ($($field:ident = $value:expr),* $(,)?) => {
        compile_error!(concat!(
                   "Entity type must be specified using @Type syntax.\n",
                   "Example usage:\n",
                   "  target_filter!(@MyEntityType,\n",
                   "      entity = MyEntity::Type,\n",
                   "      distance = \"..50\",\n",
                   "      team = \"red\"\n",
                   "  )\n",
                   "\n",
                   "The type must implement the EntityClass trait."
               ))
    };

    (@$entity_type:ty, $($field:ident = $value:expr),* $(,)?) => {{
        const _: fn() = || {
            fn assert_implements_entity_class<T: crate::entities::EntityClass>() {}
            assert_implements_entity_class::<$entity_type>();
        };

        let mut filter = crate::types::selector::TargetFilter::default();
        $(
            match stringify!($field) {
                "entity" => filter.entity = Some($value.to_string()),
                "distance" => {
                    let value_str = $value.to_string();
                    filter.distance = Some(if value_str.contains("..") {
                        let parts: Vec<&str> = value_str.split("..").collect();
                        match (parts[0], parts.get(1)) {
                            ("", Some(max)) => crate::types::world::Distance::Max(max.parse::<f32>().unwrap_or(0.0)),
                            (min, Some(&"")) => crate::types::world::Distance::Min(min.parse::<f32>().unwrap_or(0.0)),
                            (min, Some(max)) => crate::types::world::Distance::Range(
                                min.parse::<f32>().unwrap_or(0.0),
                                max.parse::<f32>().unwrap_or(0.0)
                            ),
                            _ => panic!("Invalid distance format")
                        }
                    } else {
                        crate::types::world::Distance::Exact(value_str.parse::<f32>().unwrap_or(0.0))
                    });
                },
                "team" => filter.team = Some($value.to_string()),
                "name" => filter.name = Some($value.to_string()),
                "tag" => filter.tag = Some($value.to_string()),
                "sort" => filter.sort = Some($value.to_string()),
                "nbt" => filter.nbt = Some($value.to_string()),
                "predicate" => filter.predicate = Some($value.to_string()),
                "limit" => {
                    let value_str = $value.to_string();
                    if let Ok(num) = value_str.parse::<u32>() {
                        filter.limit = Some(num);
                    }
                },
                "level" => {
                    let value_str = $value.to_string();
                    if let Ok(num) = value_str.parse::<u32>() {
                        filter.level = Some(num);
                    }
                },
                "x" | "y" | "z" | "dx" | "dy" | "dz" => {
                    let value_str = $value.to_string();
                    if let Ok(num) = value_str.parse::<f32>() {
                        match stringify!($field) {
                            "x" => filter.x = Some(num),
                            "y" => filter.y = Some(num),
                            "z" => filter.z = Some(num),
                            "dx" => filter.dx = Some(num),
                            "dy" => filter.dy = Some(num),
                            "dz" => filter.dz = Some(num),
                            _ => unreachable!()
                        }
                    }
                },
                _ => panic!("Unsupported filter field: {}", stringify!($field))
            }
        )*
        filter
    }};
}

use std::collections::HashMap;

#[macro_export]
macro_rules! bundle_enchants {
    ($($enchant:expr),* $(,)?) => {{
        let mut map: HashMap<String, u32> = HashMap::new();
        $(
            let (name, level) = $enchant.to_pair();
            map.insert(name, level);
        )*
        map
    }};
}

