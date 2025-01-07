use operation::QueryInputValueId;
use schema::{AuthorizedDirectiveId, DefinitionId, FieldDefinitionId, RequiresScopesDirectiveId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub(crate) enum QueryModifierRule {
    Authenticated,
    RequiresScopes(RequiresScopesDirectiveId),
    AuthorizedField {
        directive_id: AuthorizedDirectiveId,
        definition_id: FieldDefinitionId,
    },
    AuthorizedDefinition {
        directive_id: AuthorizedDirectiveId,
        definition_id: DefinitionId,
    },
    SkipInclude {
        // sorted
        directives: Vec<SkipIncludeDirective>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum SkipIncludeDirective {
    SkipIf(QueryInputValueId),
    IncludeIf(QueryInputValueId),
}
