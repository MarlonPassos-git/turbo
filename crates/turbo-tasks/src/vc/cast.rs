use std::marker::PhantomData;

use anyhow::Result;

use crate::{backend::CellContent, ReadRef, TraitRef, VcValueTrait, VcValueType};

/// Trait defined to share behavior between values and traits within
/// [`ReadRawVcFuture`]. See [`ValueCast`] and [`TraitCast`].
///
/// This should not be implemented by users.
pub trait VcCast {
    type Output;

    fn cast(content: CellContent) -> Result<Self::Output>;
}

/// Casts an arbitrary cell content into a [`ReadRef<T, U>`].
pub struct VcValueTypeCast<T> {
    _phantom: PhantomData<T>,
}

impl<T> VcCast for VcValueTypeCast<T>
where
    T: VcValueType,
{
    type Output = ReadRef<T>;

    fn cast(content: CellContent) -> Result<Self::Output> {
        content.cast::<T>()
    }
}

/// Casts an arbitrary cell content into a [`TraitRef<T>`].
pub struct VcValueTraitCast<T>
where
    T: ?Sized,
{
    _phantom: PhantomData<T>,
}

impl<T> VcCast for VcValueTraitCast<T>
where
    T: VcValueTrait + ?Sized,
{
    type Output = TraitRef<T>;

    fn cast(content: CellContent) -> Result<Self::Output> {
        // Safety: Constructor ensures the cell content points to a value that
        // implements T
        content.cast_trait::<T>()
    }
}
