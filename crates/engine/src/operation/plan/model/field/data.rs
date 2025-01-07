use operation::Location;
use schema::FieldDefinition;
use walker::{Iter, Walk};

use crate::{
    operation::{
        CachedOperationContext, DataFieldId, DataFieldRecord, FieldArgument, Location, OperationPlanContext,
        PartitionDataField, PartitionDataFieldId, PartitionDataFieldRecord, PartitionFieldArguments, PlanSelectionSet,
    },
    response::PositionedResponseKey,
};

#[derive(Clone, Copy)]
pub(crate) struct PlanDataField<'a> {
    pub(in crate::operation::plan::model) ctx: OperationPlanContext<'a>,
    pub(in crate::operation::plan::model) id: PartitionDataFieldId,
}

#[allow(unused)]
impl<'a> PlanDataField<'a> {
    #[allow(clippy::should_implement_trait)]
    fn as_ref(&self) -> &'a PartitionDataFieldRecord {
        &self.ctx.logical_plan[self.id]
    }
    pub(crate) fn subgraph_response_key_str(&self) -> &'a str {
        let record = self.as_ref();
        let key = record.subgraph_key.unwrap_or(record.key);
        &self.ctx.operation.response_keys[key]
    }
    pub(crate) fn location(&self) -> Location {
        self.as_ref().location
    }
    pub(crate) fn definition(&self) -> FieldDefinition<'a> {
        self.as_ref().definition_id.walk(self.ctx.schema)
    }
    pub(crate) fn arguments(&self) -> PartitionFieldArguments<'a> {
        self.as_ref().argument_ids.walk(self.ctx)
    }
    pub(crate) fn selection_set(&self) -> PlanSelectionSet<'a> {
        let field = self.as_ref();
        PlanSelectionSet {
            ctx: self.ctx,
            item: field.selection_set_record,
            requires_typename: field.selection_set_requires_typename,
        }
    }
}

impl std::fmt::Debug for PlanDataField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlanDataField")
            .field("key", &self.subgraph_response_key_str())
            .field("location", &self.location())
            .field("definition", &self.definition())
            .field("arguments", &self.arguments())
            .field("selection_set", &self.selection_set())
            .finish()
    }
}
