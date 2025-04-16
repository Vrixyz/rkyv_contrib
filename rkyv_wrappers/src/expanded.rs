mod doctest {
    use crate::custom_phantom::CustomPhantom;
    use rkyv::{Archive, Portable};
    use std::marker::PhantomData;
    #[repr(C)]
    #[rkyv(as = StructWithPhantom<T>)]
    struct StructWithPhantom<T> {
        #[rkyv(with = CustomPhantom<T>)]
        pub phantom: PhantomData<T>,
    }
    unsafe impl<T> ::rkyv::Portable for StructWithPhantom<T> where PhantomData<T>: ::rkyv::Portable {}
    #[automatically_derived]
    ///The resolver for an archived [`StructWithPhantom`]
    struct StructWithPhantomResolver<T>
    where
        CustomPhantom<T>: ::rkyv::with::ArchiveWith<PhantomData<T>>,
    {
        phantom: <CustomPhantom<T> as ::rkyv::with::ArchiveWith<PhantomData<T>>>::Resolver,
    }
    impl<T> ::rkyv::Archive for StructWithPhantom<T>
    where
        CustomPhantom<T>: ::rkyv::with::ArchiveWith<PhantomData<T>>,
    {
        type Archived = StructWithPhantom<T>;
        type Resolver = StructWithPhantomResolver<T>;
        #[allow(clippy::unit_arg)]
        fn resolve(&self, resolver: Self::Resolver, out: ::rkyv::Place<Self::Archived>) {
            let field_ptr = unsafe { &raw mut (*out.ptr()).phantom };
            let field_out = unsafe { ::rkyv::Place::from_field_unchecked(out, field_ptr) };
            <CustomPhantom<T> as ::rkyv::with::ArchiveWith<PhantomData<T>>>::resolve_with(
                &self.phantom,
                resolver.phantom,
                field_out,
            );
        }
    }
}
