mod proxy;
mod query;

pub use {
    query::*,
    proxy::*
};

pub trait Context {
    type Query<I: QueryItem>: Query<I>;

    fn query<I: QueryItem>(&self) -> Self::Query<I>;
}