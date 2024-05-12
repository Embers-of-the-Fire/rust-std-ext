use std_ext::tuple::{TupleConcat, TupleInsert, TupleRemove, TupleZip};

#[test]
fn test_tuple_zip() {
    let x = (1u8, 2usize, false);
    let y = x.zip('a');

    assert_eq!(y, (1, 2, false, 'a'))
}

#[test]
fn test_tuple_insert() {
    let x = (1u8, 2usize, false);
    let y = x.insert::<1>('a');

    assert_eq!(y, (1, 'a', 2, false))
}

#[test]
fn test_tuple_remove() {
    let x = (1u8, 2usize, false);
    let y = x.remove::<2>();
    assert_eq!(y, (1, 2))
}

#[test]
fn test_tuple_concat() {
    let x = (1u8, 2usize, false);
    let y = x.concat((10i32, 'a'));
    assert_eq!(y, (1, 2, false, 10, 'a'));
}
