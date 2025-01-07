//! ===================
//! !!! DO NOT EDIT !!!
//! ===================
//! Generated with: `cargo run -p engine-codegen`
//! Source file: <engine-codegen dir>/domain/query_plan.graphql
use crate::prepare::cached::query_plan::model::{
    prelude::*, PartitionDataField, PartitionDataFieldId, PartitionTypenameField, PartitionTypenameFieldId,
};
use walker::{Iter, Walk};

/// Generated from:
///
/// ```custom,{.language-graphql}
/// type SelectionSet @meta(module: "selection_set", derive: ["Default"]) @copy {
///   data_fields_ordered_by_parent_entity_id_then_key: [PartitionDataField!]!
///     @field(record_field_name: "data_field_ids_ordered_by_parent_entity_id_then_key")
///   typename_fields_ordered_by_type_condition_id_then_key: [PartitionTypenameField!]!
///     @field(record_field_name: "typename_field_ids_ordered_by_type_condition_id_then_key")
/// }
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Copy)]
pub(crate) struct SelectionSetRecord {
    pub data_field_ids_ordered_by_parent_entity_id_then_key: IdRange<PartitionDataFieldId>,
    pub typename_field_ids_ordered_by_type_condition_id_then_key: IdRange<PartitionTypenameFieldId>,
}

#[derive(Clone, Copy)]
pub(crate) struct SelectionSet<'a> {
    pub(in crate::prepare::cached::query_plan::model) ctx: CachedOperationContext<'a>,
    pub(in crate::prepare::cached::query_plan::model) item: SelectionSetRecord,
}

impl std::ops::Deref for SelectionSet<'_> {
    type Target = SelectionSetRecord;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

#[allow(unused)]
impl<'a> SelectionSet<'a> {
    #[allow(clippy::should_implement_trait)]
    pub(crate) fn as_ref(&self) -> &SelectionSetRecord {
        &self.item
    }
    pub(crate) fn data_fields_ordered_by_parent_entity_id_then_key(
        &self,
    ) -> impl Iter<Item = PartitionDataField<'a>> + 'a {
        self.as_ref()
            .data_field_ids_ordered_by_parent_entity_id_then_key
            .walk(self.ctx)
    }
    pub(crate) fn typename_fields_ordered_by_type_condition_id_then_key(
        &self,
    ) -> impl Iter<Item = PartitionTypenameField<'a>> + 'a {
        self.as_ref()
            .typename_field_ids_ordered_by_type_condition_id_then_key
            .walk(self.ctx)
    }
}

#[allow(unused)]
impl<'a> Walk<CachedOperationContext<'a>> for SelectionSetRecord {
    type Walker<'w>
        = SelectionSet<'w>
    where
        'a: 'w;
    fn walk<'w>(self, ctx: impl Into<CachedOperationContext<'a>>) -> Self::Walker<'w>
    where
        Self: 'w,
        'a: 'w,
    {
        SelectionSet {
            ctx: ctx.into(),
            item: self,
        }
    }
}

impl std::fmt::Debug for SelectionSet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionSet")
            .field(
                "data_fields_ordered_by_parent_entity_id_then_key",
                &self.data_fields_ordered_by_parent_entity_id_then_key(),
            )
            .field(
                "typename_fields_ordered_by_type_condition_id_then_key",
                &self.typename_fields_ordered_by_type_condition_id_then_key(),
            )
            .finish()
    }
}
