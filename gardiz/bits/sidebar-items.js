initSidebarItems({"trait":[["CastSigned","Trait for bit-casts from unsigned integers to 2’s complement signed integers."],["CastUnsigned","Trait for bit-casts from 2’s complement signed integers to unsigned integers."],["Distance","Trait for computing absolute distance between two numbers. In general, types should not worry with this trait, but instead implement [`Sub`] and [`Ord`]."],["ExcessToSigned","Trait for converting unsigned numbers into signed numbers, but instead of a true bit-cast, this conversion should treat the unsigned number as “excess of N” number. So, conversion should be equivalent to this: `i = u - N` (i.e. `N` becomes the new zero)."],["HalfExcess","Trait for getting the “excess” that is the half of an unsigned type’s maximum value, typically `0111...1111`. Types should not worry with this trait, but instead implement [`Unsigned`] and [`Bounded`], since there is a blank implementation for them, and there is no other way to implement the trait (`Unsigned` and `Bounded` are super traits of this trait)."],["SignedToExcess","Trait for converting signed numbers into unsigned numbers, but instead of a true bit-cast, this conversion should treat the unsigned number as “excess of N” number. So, conversion should be equivalent to this: `u = i + N` (i.e. `N` becomes the new zero)."]]});