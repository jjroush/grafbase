//! ===================
//! !!! DO NOT EDIT !!!
//! ===================
//! Generated with: `cargo run -p engine-codegen`
//! Source file: <engine-codegen dir>/domain/query_plan.graphql
use crate::prepare::cached::query_plan::{
    prelude::*, PartitionDataField, PartitionDataFieldId, PartitionTypenameField, PartitionTypenameFieldId,
};
use walker::Walk;

/// Generated from:
///
/// ```custom,{.language-graphql}
/// union PartitionField @id @meta(module: "field") @variants(remove_suffix: "Field") =
///   | PartitionDataField
///   | PartitionTypenameField
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PartitionFieldId {
    PartitionData(PartitionDataFieldId),
    PartitionTypename(PartitionTypenameFieldId),
}

impl std::fmt::Debug for PartitionFieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionFieldId::PartitionData(variant) => variant.fmt(f),
            PartitionFieldId::PartitionTypename(variant) => variant.fmt(f),
        }
    }
}

impl From<PartitionDataFieldId> for PartitionFieldId {
    fn from(value: PartitionDataFieldId) -> Self {
        PartitionFieldId::PartitionData(value)
    }
}
impl From<PartitionTypenameFieldId> for PartitionFieldId {
    fn from(value: PartitionTypenameFieldId) -> Self {
        PartitionFieldId::PartitionTypename(value)
    }
}

#[allow(unused)]
impl PartitionFieldId {
    pub(crate) fn is_partition_data(&self) -> bool {
        matches!(self, PartitionFieldId::PartitionData(_))
    }
    pub(crate) fn as_partition_data(&self) -> Option<PartitionDataFieldId> {
        match self {
            PartitionFieldId::PartitionData(id) => Some(*id),
            _ => None,
        }
    }
    pub(crate) fn is_partition_typename(&self) -> bool {
        matches!(self, PartitionFieldId::PartitionTypename(_))
    }
    pub(crate) fn as_partition_typename(&self) -> Option<PartitionTypenameFieldId> {
        match self {
            PartitionFieldId::PartitionTypename(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum PartitionField<'a> {
    PartitionData(PartitionDataField<'a>),
    PartitionTypename(PartitionTypenameField<'a>),
}

impl std::fmt::Debug for PartitionField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartitionField::PartitionData(variant) => variant.fmt(f),
            PartitionField::PartitionTypename(variant) => variant.fmt(f),
        }
    }
}

impl<'a> Walk<CachedOperationContext<'a>> for PartitionFieldId {
    type Walker<'w>
        = PartitionField<'w>
    where
        'a: 'w;
    fn walk<'w>(self, ctx: impl Into<CachedOperationContext<'a>>) -> Self::Walker<'w>
    where
        Self: 'w,
        'a: 'w,
    {
        let ctx: CachedOperationContext<'a> = ctx.into();
        match self {
            PartitionFieldId::PartitionData(id) => PartitionField::PartitionData(id.walk(ctx)),
            PartitionFieldId::PartitionTypename(id) => PartitionField::PartitionTypename(id.walk(ctx)),
        }
    }
}

#[allow(unused)]
impl<'a> PartitionField<'a> {
    pub(crate) fn id(&self) -> PartitionFieldId {
        match self {
            PartitionField::PartitionData(walker) => PartitionFieldId::PartitionData(walker.id),
            PartitionField::PartitionTypename(walker) => PartitionFieldId::PartitionTypename(walker.id),
        }
    }
    pub(crate) fn is_partition_data(&self) -> bool {
        matches!(self, PartitionField::PartitionData(_))
    }
    pub(crate) fn as_partition_data(&self) -> Option<PartitionDataField<'a>> {
        match self {
            PartitionField::PartitionData(item) => Some(*item),
            _ => None,
        }
    }
    pub(crate) fn is_partition_typename(&self) -> bool {
        matches!(self, PartitionField::PartitionTypename(_))
    }
    pub(crate) fn as_partition_typename(&self) -> Option<PartitionTypenameField<'a>> {
        match self {
            PartitionField::PartitionTypename(item) => Some(*item),
            _ => None,
        }
    }
}
