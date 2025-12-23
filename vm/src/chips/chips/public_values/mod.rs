use std::marker::PhantomData;

pub mod columns;
mod constraints;
mod traces;

pub(crate) const PUB_VALUES_LOG_HEIGHT: usize = 4;
#[derive(Default)]
pub struct PublicValuesChip<F> {
    _phantom: PhantomData<F>,
}
