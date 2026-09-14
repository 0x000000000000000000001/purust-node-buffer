// unsafeFreeze/unsafeThaw/slice preserve the underlying Node buffer identity.
// ImmutableBuffer's native declaration owns the shared representation.
pub type Buffer = Purs_Node_Buffer_Immutable::ImmutableBuffer;
