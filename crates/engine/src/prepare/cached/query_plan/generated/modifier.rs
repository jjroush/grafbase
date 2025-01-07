//! ===================
//! !!! DO NOT EDIT !!!
//! ===================
//! Generated with: `cargo run -p engine-codegen`
//! Source file: <engine-codegen dir>/domain/query_plan.graphql
use crate::prepare::cached::query_plan::{
    generated::{PartitionField, PartitionFieldId},
    prelude::*,
    QueryModifierRule,
};
use walker::{Iter, Walk};

/// Generated from:
///
/// ```custom,{.language-graphql}
/// type QueryModifierDefinition @meta(module: "modifier") {
///   rule: QueryModifierRule!
///   impacts_root_object: Boolean!
///   impacted_fields: [PartitionField!]!
/// }
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct QueryModifierDefinitionRecord {
    pub rule: QueryModifierRule,
    pub impacts_root_object: bool,
    pub impacted_field_ids: Vec<PartitionFieldId>,
}

#[derive(Clone, Copy)]
pub(crate) struct QueryModifierDefinition<'a> {
    pub(in crate::prepare::cached::query_plan) ctx: CachedOperationContext<'a>,
    pub(in crate::prepare::cached::query_plan) ref_: &'a QueryModifierDefinitionRecord,
}

impl std::ops::Deref for QueryModifierDefinition<'_> {
    type Target = QueryModifierDefinitionRecord;
    fn deref(&self) -> &Self::Target {
        self.ref_
    }
}

#[allow(unused)]
impl<'a> QueryModifierDefinition<'a> {
    #[allow(clippy::should_implement_trait)]
    pub(crate) fn as_ref(&self) -> &'a QueryModifierDefinitionRecord {
        self.ref_
    }
    pub(crate) fn impacted_fields(&self) -> impl Iter<Item = PartitionField<'a>> + 'a {
        self.as_ref().impacted_field_ids.walk(self.ctx)
    }
}

impl<'a> Walk<CachedOperationContext<'a>> for &QueryModifierDefinitionRecord {
    type Walker<'w>
        = QueryModifierDefinition<'w>
    where
        Self: 'w,
        'a: 'w;
    fn walk<'w>(self, ctx: impl Into<CachedOperationContext<'a>>) -> Self::Walker<'w>
    where
        Self: 'w,
        'a: 'w,
    {
        QueryModifierDefinition {
            ctx: ctx.into(),
            ref_: self,
        }
    }
}

impl std::fmt::Debug for QueryModifierDefinition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryModifierDefinition")
            .field("rule", &self.rule)
            .field("impacts_root_object", &self.impacts_root_object)
            .field("impacted_fields", &self.impacted_fields())
            .finish()
    }
}
