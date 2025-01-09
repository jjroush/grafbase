use operation::{Location, PositionedResponseKey, TypenameFieldId, TypenameFieldRecord};
use schema::CompositeType;
use walker::Walk;

use crate::prepare::{OperationPlanContext, PartitionTypenameFieldId, PartitionTypenameFieldRecord};

#[derive(Clone, Copy)]
pub(crate) struct PlanTypenameField<'a> {
    pub(in crate::prepare::operation_plan::model) ctx: OperationPlanContext<'a>,
    pub(in crate::prepare::operation_plan::model) id: PartitionTypenameFieldId,
}

#[allow(unused)]
impl<'a> PlanTypenameField<'a> {
    #[allow(clippy::should_implement_trait)]
    fn as_ref(&self) -> &'a PartitionTypenameFieldRecord {
        &self.ctx.cached.query_plan[self.id]
    }
    pub(crate) fn key(&self) -> PositionedResponseKey {
        let field = self.as_ref();
        field.key.with_position(field.query_position)
    }
    pub(crate) fn location(&self) -> Location {
        self.as_ref().location
    }
    pub(crate) fn type_condition(&self) -> CompositeType<'a> {
        todo!()
    }
}

impl std::fmt::Debug for PlanTypenameField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypenameField")
            .field("key", &self.key())
            .field("location", &self.location())
            .field("type_condition", &self.type_condition())
            .finish()
    }
}
