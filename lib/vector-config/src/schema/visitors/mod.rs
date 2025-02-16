mod inline_single;
pub mod merge;
pub mod scoped_visit;
mod unevaluated;

#[cfg(test)]
pub(self) mod test;

pub use self::inline_single::InlineSingleUseReferencesVisitor;
pub use self::unevaluated::DisallowUnevaluatedPropertiesVisitor;
