//! Syntatic analysis of EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html)
//!
//! This module is based on [nom](https://github.com/Geal/nom) parser combinater.

mod entity;
mod expression;
mod literal;
mod schema;
mod simple_data_type;

pub use entity::*;
pub use expression::*;
pub use literal::*;
pub use schema::*;
pub use simple_data_type::*;

use derive_more::{Deref, Display};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

/// 128 letter = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` |`m` | `n` | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` |`y` | `z` .
pub fn letter(input: &str) -> IResult<&str, char> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

/// 124 digit = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> IResult<&str, char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

/// 143 simple_id = letter { letter | digit | `_` } .
pub fn simple_id(input: &str) -> IResult<&str, String> {
    tuple((letter, many0(alt((letter, digit, char('_'))))))
        .map(|(head, tail)| format!("{}{}", head, tail.into_iter().collect::<String>()))
        .parse(input)
}

macro_rules! define_id_ref {
    ($ID:ident, $Ref:ident, $id_parse:ident, $ref_parse:ident) => {
        #[derive(Debug, Clone, PartialEq, Display, Deref)]
        pub struct $ID(String);

        #[derive(Debug, Clone, PartialEq, Display, Deref)]
        pub struct $Ref(String);

        pub fn $id_parse(input: &str) -> IResult<&str, $ID> {
            simple_id.map(|id| $ID(id)).parse(input)
        }

        pub fn $ref_parse(input: &str) -> IResult<&str, $Ref> {
            simple_id.map(|id| $Ref(id)).parse(input)
        }
    };
}

define_id_ref!(AttributeID, AttributeRef, attribute_id, attribute_ref);
define_id_ref!(ConstantID, ConstantRef, constant_id, constant_ref);
define_id_ref!(EntityID, EntityRef, entity_id, entity_ref);
define_id_ref!(
    EnumerationID,
    EnumerationRef,
    enumeration_id,
    enumeration_ref
);
define_id_ref!(FunctionID, FunctionRef, function_id, function_ref);
define_id_ref!(ParameterID, ParameterRef, parameter_id, parameter_ref);
define_id_ref!(ProcedureID, ProcedureRef, procedure_id, procedure_ref);
define_id_ref!(RenameID, RenameRef, rename_id, rename_ref);
define_id_ref!(RuleID, RuleRef, rule_id, rule_ref);
define_id_ref!(RuleLabelID, RuleLabelRef, rule_label_id, rule_label_ref);
define_id_ref!(SchemaID, SchemaRef, schema_id, schema_ref);
define_id_ref!(
    SubtypeConstraintID,
    SubtypeConstraintRef,
    subtype_constraint_id,
    subtype_constraint_ref
);
define_id_ref!(TypeID, TypeRef, type_id, type_ref);
define_id_ref!(TypeLabelID, TypeLabelRef, type_label_id, type_label_ref);
define_id_ref!(VariableID, VariableRef, variable_id, variable_ref);

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn letter() {
        let (residual, l) = super::letter("h").finish().unwrap();
        assert_eq!(l, 'h');
        assert_eq!(residual, "");

        let (residual, l) = super::letter("abc").finish().unwrap();
        assert_eq!(l, 'a');
        assert_eq!(residual, "bc");

        // Capital is not allowed
        assert!(super::letter("H").finish().is_err());
        // Number is not allowed
        assert!(super::letter("2").finish().is_err());
    }

    #[test]
    fn digit() {
        let (residual, l) = super::digit("123").finish().unwrap();
        assert_eq!(l, '1');
        assert_eq!(residual, "23");

        // Alphabets are not allowed
        assert!(super::digit("h").finish().is_err());
    }

    #[test]
    fn simple_id_valid() {
        let (residual, id) = super::simple_id("h").finish().unwrap();
        assert_eq!(id, "h");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("homhom").finish().unwrap();
        assert_eq!(id, "homhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("ho_mhom").finish().unwrap();
        assert_eq!(id, "ho_mhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("h10o_1mh2om").finish().unwrap();
        assert_eq!(id, "h10o_1mh2om");
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_id_invalid() {
        // Capital is not allowed
        assert!(super::simple_id("HomHom").finish().is_err());
        // `_` cannot use as first
        assert!(super::simple_id("_homhom").finish().is_err());
        // digit cannot use as first
        assert!(super::simple_id("1homhom").finish().is_err());
        // Empty is invlaid
        assert!(super::simple_id("").finish().is_err());
    }
}