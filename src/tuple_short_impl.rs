__impl_tuple_zip!(A | Z);
__impl_tuple_zip!(A, B | Z);
__impl_tuple_zip!(A, B, C | Z);
__impl_tuple_zip!(A, B, C, D | Z);
__impl_tuple_zip!(A, B, C, D, E | Z);

__impl_tuple_insert!(A | Z);
__impl_tuple_insert!(A, B | Z);
__impl_tuple_insert!(A, B, C | Z);
__impl_tuple_insert!(A, B, C, D | Z);
__impl_tuple_insert!(A, B, C, D, E | Z);

__impl_tuple_insert_exact!(A | Z at 0 => Z, A);
__impl_tuple_insert_exact!(A | Z at 1 => A, Z);
__impl_tuple_insert_exact!(A, B | Z at 0 => Z, A, B);
__impl_tuple_insert_exact!(A, B | Z at 1 => A, Z, B);
__impl_tuple_insert_exact!(A, B | Z at 2 => A, B, Z);
__impl_tuple_insert_exact!(A, B, C | Z at 0 => Z, A, B, C);
__impl_tuple_insert_exact!(A, B, C | Z at 1 => A, Z, B, C);
__impl_tuple_insert_exact!(A, B, C | Z at 2 => A, B, Z, C);
__impl_tuple_insert_exact!(A, B, C | Z at 3 => A, B, C, Z);
__impl_tuple_insert_exact!(A, B, C, D | Z at 0 => Z, A, B, C, D);
__impl_tuple_insert_exact!(A, B, C, D | Z at 1 => A, Z, B, C, D);
__impl_tuple_insert_exact!(A, B, C, D | Z at 2 => A, B, Z, C, D);
__impl_tuple_insert_exact!(A, B, C, D | Z at 3 => A, B, C, Z, D);
__impl_tuple_insert_exact!(A, B, C, D | Z at 4 => A, B, C, D, Z);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 0 => Z, A, B, C, D, E);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 1 => A, Z, B, C, D, E);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 2 => A, B, Z, C, D, E);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 3 => A, B, C, Z, D, E);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 4 => A, B, C, D, Z, E);
__impl_tuple_insert_exact!(A, B, C, D, E | Z at 5 => A, B, C, D, E, Z);

__impl_tuple_remove!(A);
__impl_tuple_remove!(A, B);
__impl_tuple_remove!(A, B, C);
__impl_tuple_remove!(A, B, C, D);
__impl_tuple_remove!(A, B, C, D, E);

// special case for (A,)
impl<A> TupleRemoveExact<0> for (A,) {
    type Output = ();

    #[allow(unused_variables)]
    fn remove(self) -> Self::Output {
    }
}
__impl_tuple_remove_exact!(A, B at 0 => B);
__impl_tuple_remove_exact!(A, B at 1 => A);
__impl_tuple_remove_exact!(A, B, C at 0 => B, C);
__impl_tuple_remove_exact!(A, B, C at 1 => A, C);
__impl_tuple_remove_exact!(A, B, C at 2 => A, B);
__impl_tuple_remove_exact!(A, B, C, D at 0 => B, C, D);
__impl_tuple_remove_exact!(A, B, C, D at 1 => A, C, D);
__impl_tuple_remove_exact!(A, B, C, D at 2 => A, B, D);
__impl_tuple_remove_exact!(A, B, C, D at 3 => A, B, C);
__impl_tuple_remove_exact!(A, B, C, D, E at 0 => B, C, D, E);
__impl_tuple_remove_exact!(A, B, C, D, E at 1 => A, C, D, E);
__impl_tuple_remove_exact!(A, B, C, D, E at 2 => A, B, D, E);
__impl_tuple_remove_exact!(A, B, C, D, E at 3 => A, B, C, E);
__impl_tuple_remove_exact!(A, B, C, D, E at 4 => A, B, C, D);

__impl_tuple_concat!(A with B);
__impl_tuple_concat!(A with B, C);
__impl_tuple_concat!(A with B, C, D);
__impl_tuple_concat!(A with B, C, D, E);
__impl_tuple_concat!(A with B, C, D, E, F);
__impl_tuple_concat!(A with B, C, D, E, F, G);
__impl_tuple_concat!(A with B, C, D, E, F, G, H);
__impl_tuple_concat!(A with B, C, D, E, F, G, H, I);
__impl_tuple_concat!(A with B, C, D, E, F, G, H, I, J);
__impl_tuple_concat!(A, B with C);
__impl_tuple_concat!(A, B with C, D);
__impl_tuple_concat!(A, B with C, D, E);
__impl_tuple_concat!(A, B with C, D, E, F);
__impl_tuple_concat!(A, B with C, D, E, F, G);
__impl_tuple_concat!(A, B with C, D, E, F, G, H);
__impl_tuple_concat!(A, B with C, D, E, F, G, H, I);
__impl_tuple_concat!(A, B with C, D, E, F, G, H, I, J);
__impl_tuple_concat!(A, B, C with D);
__impl_tuple_concat!(A, B, C with D, E);
__impl_tuple_concat!(A, B, C with D, E, F);
__impl_tuple_concat!(A, B, C with D, E, F, G);
__impl_tuple_concat!(A, B, C with D, E, F, G, H);
__impl_tuple_concat!(A, B, C with D, E, F, G, H, I);
__impl_tuple_concat!(A, B, C with D, E, F, G, H, I, J);
__impl_tuple_concat!(A, B, C, D with E);
__impl_tuple_concat!(A, B, C, D with E, F);
__impl_tuple_concat!(A, B, C, D with E, F, G);
__impl_tuple_concat!(A, B, C, D with E, F, G, H);
__impl_tuple_concat!(A, B, C, D with E, F, G, H, I);
__impl_tuple_concat!(A, B, C, D with E, F, G, H, I, J);
__impl_tuple_concat!(A, B, C, D, E with F);
__impl_tuple_concat!(A, B, C, D, E with F, G);
__impl_tuple_concat!(A, B, C, D, E with F, G, H);
__impl_tuple_concat!(A, B, C, D, E with F, G, H, I);
__impl_tuple_concat!(A, B, C, D, E with F, G, H, I, J);
