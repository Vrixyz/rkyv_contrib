//! A wrapper that converts a `Vec` to an `ArchivedHashMap` at serialization time.

use rkyv::{
    rancor::Fallible,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Place,
};
use std::marker::PhantomData;

/// A wrapper that allows for changing the generic type of a PhantomData<T>.
///
/// Example:
///
/// ```rust
/// use std::marker::PhantomData;
/// use rkyv::{
///     deserialize, bytecheck, Portable, Archive, Deserialize, Serialize, rancor::Infallible, vec::ArchivedVec, Archived, with::With,
///     rancor::Error,
/// };
/// use rkyv_wrappers::custom_phantom::CustomPhantom;
/// use rkyv::with::ArchiveWith;
/// #[repr(C)]
/// #[derive(Archive, Serialize, Deserialize, bytecheck::CheckBytes, Debug, PartialEq, Eq, Default)]
/// #[rkyv(compare(PartialEq), derive(Debug))]
/// // This should not be necessary, but somehow rustc needs a push in the right direction.
/// #[rkyv(archive_bounds(CustomPhantom<T>: ArchiveWith<PhantomData<T>, Archived = PhantomData<T>>))]
/// struct StructWithPhantom<T> {
/// 	pub num: i32,
///     #[rkyv(with = CustomPhantom<T>)]
///     pub phantom: PhantomData<T>,
/// }
/// let value = StructWithPhantom::<Vec<i32>>::default();
/// let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
/// let archived = rkyv::access::<ArchivedStructWithPhantom<Vec<i32>>, Error>(&bytes[..]).unwrap();
///  assert_eq!(archived, &value);
///
/// let new_value = StructWithPhantom::<Vec<i32>> {
///     num: 42,
///     phantom: PhantomData,
/// };
///
/// let deserialized = deserialize::<StructWithPhantom<Vec<i32>>, Error>(archived).unwrap();
/// assert_eq!(&deserialized, &value);
/// ```
pub struct CustomPhantom<NT: ?Sized> {
    _data: PhantomData<*const NT>,
}

impl<OT: ?Sized, NT: ?Sized> ArchiveWith<PhantomData<OT>> for CustomPhantom<NT> {
    type Archived = PhantomData<NT>;
    type Resolver = ();

    #[inline]
    fn resolve_with(_: &PhantomData<OT>, _: Self::Resolver, _: Place<Self::Archived>) {}
}

impl<OT: ?Sized, NT: ?Sized, S: Fallible + ?Sized> SerializeWith<PhantomData<OT>, S>
    for CustomPhantom<NT>
{
    #[inline]
    fn serialize_with(_: &PhantomData<OT>, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<OT: ?Sized, NT: ?Sized, D: Fallible + ?Sized>
    DeserializeWith<PhantomData<NT>, PhantomData<OT>, D> for CustomPhantom<NT>
{
    #[inline]
    fn deserialize_with(_: &PhantomData<NT>, _: &mut D) -> Result<PhantomData<OT>, D::Error> {
        Ok(PhantomData)
    }
}
