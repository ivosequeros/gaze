use super::selector::Selector;

pub enum Constraint {
    Equal(Vec<u8>),
    GreaterThan(Vec<u8>),
    LowerThan(Vec<u8>),
    StartsWith(Vec<u8>),
    EndsWith(Vec<u8>),
}

pub type FieldConstraint = (usize, Constraint);
pub type TypeConstraints = (Vec<u8>, Vec<FieldConstraint>);
pub type Filter = Vec<TypeConstraints>;

pub type ChecksRequiredPerType = HashMap<Vec<u8>, usize>;

pub trait FilterBuilder {
    fn build_checks_per_type(self);
    fn parse_constraint(input: serde_json::Value) -> Result<Constraint, ()>;
    fn parse(input: serde_json::Value) -> Result<Filter, ()>;
}

impl FilterBuilder for Filter {
    fn build_checks_per_type(self) {
        let checks = ChecksRequiredPerType::new();
        for (message_type, constraints) in self {
            checks.insert(message_type, constraints.len());
        }

        checks
    }
    fn parse_constraint(input: serde_json::Value) -> Result<Constraint, ()> {
        match input {
            serde_json::Value::Object(object_values) => {
                if object_values.len() > 1 {
                    return Err(());
                }
                let Some((operator, value)) = object_values.iter().next();

                let Constraint::Equal(parsed_value) = Self::parse_constraint(*value).unwrap();
                match &operator[..] {
                    "$lt" => Ok(Constraint::LowerThan(parsed_value)),
                    "$gt" => Ok(Constraint::GreaterThan(parsed_value)),
                    _ => return Err(()),
                }
            }
            serde_json::Value::Number(number) if number.is_u64() => {
                let value = number.as_u64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            serde_json::Value::Number(number) if number.is_i64() => {
                let value = number.as_i64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            serde_json::Value::Number(number) if number.is_f64() => {
                let value = number.as_f64().unwrap().to_le_bytes().to_vec();
                Ok(Constraint::Equal(value))
            }
            _ => return Err(()),
        }
    }
    fn parse(input: serde_json::Value) -> Result<Filter, ()> {
        let filter = Vec::new();
        /* Check original filter: */
        let raw_contraints_per_type = match input {
            serde_json::Value::Array(values) => values,
            _ => return Err(()),
        };
        /* Build filter: */
        for raw_type_constraints in raw_contraints_per_type {
            let (message_type, fields) = match raw_type_constraints {
                serde_json::Value::Object(values) => {
                    let message_type = match values.remove("$type") {
                        Some(message_type) => message_type,
                        None => return Err(()),
                    };

                    (message_type, values)
                }
                _ => return Err(()),
            };

            /* For each remaining field: */
            let type_constraints = (
                message_type,
                fields.iter().map(|(field_name, value)| {
                    (field_name, Self::parse_constraint(*value).unwrap())
                }),
            );
        }

        Ok(filter)
    }
}
