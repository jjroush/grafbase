mod builder;
mod error;
mod query_plan;

use grafbase_telemetry::graphql::{GraphqlOperationAttributes, OperationType};
use id_newtypes::IdRange;
use operation::{Operation, OperationAttributes, OperationContext};
use schema::Schema;
use walker::{Iter, Walk};

pub(crate) use error::*;
pub(crate) use query_plan::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct CachedOperation {
    pub operation: Operation,
    pub query_plan: QueryPlan,
}

/// Solving is divided in roughly three steps:
/// 1. Run the query_solver crate to generate the SolutionGraph, defining what resolver to use for
///    which part of query and all the field dependencies.
/// 2. Take the SolutionGraph and the BoundOperation to create all the QueryPartitions in the SolvedOperation
/// 3. Compute all the field shapes for each partition.
pub(crate) fn solve(schema: &Schema, operation: Operation) -> SolveResult<CachedOperation> {
    builder::Solver::build(schema, operation)?.solve()
}

#[derive(Clone, Copy)]
pub(crate) struct CachedOperationContext<'a> {
    pub schema: &'a Schema,
    pub cached: &'a CachedOperation,
}

impl<'a> From<CachedOperationContext<'a>> for &'a Schema {
    fn from(ctx: CachedOperationContext<'a>) -> Self {
        ctx.schema
    }
}

impl<'a> From<CachedOperationContext<'a>> for OperationContext<'a> {
    fn from(ctx: CachedOperationContext<'a>) -> Self {
        OperationContext {
            schema: ctx.schema,
            operation: &ctx.cached.operation,
        }
    }
}

impl<'a> CachedOperationContext<'a> {
    pub(in crate::prepare::cached) fn query_partitions(&self) -> impl Iter<Item = QueryPartition<'a>> + 'a {
        IdRange::<QueryPartitionId>::from(0..self.cached.query_plan.partitions.len()).walk(*self)
    }

    // pub(in crate::operation) fn response_modifier_rules(
    //     &self,
    // ) -> impl Iter<Item = (ResponseModifierRule, impl Iterator<Item = DataField<'a>> + 'a)> + 'a {
    //     let ctx = *self;
    //     self.operation
    //         .response_modifier_rule_to_impacted_fields
    //         .iter()
    //         .map(move |item| (item.rule, item.impacted_field_ids.walk(ctx)))
    // }
}

impl CachedOperation {
    pub(crate) fn ty(&self) -> OperationType {
        self.operation.attributes.ty
    }

    /// Should be used when a request has errored and we only have the cached attributes
    pub(crate) fn operation_attributes_for_error(&self) -> GraphqlOperationAttributes {
        let OperationAttributes {
            ty,
            name,
            sanitized_query,
        } = self.operation.attributes.clone();

        GraphqlOperationAttributes {
            ty,
            name,
            sanitized_query,
            complexity_cost: None,
        }
    }
}
