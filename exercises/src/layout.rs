/*
       Even if Foo is aligned, the 1 byte we allocated to tiny is going to make normal miss its alignment.
       To rectify this, the compiler inserts 3 bytes of padding. No values go into the padding, but it does take up space.
       Padding = bytes with an indeterminate value that are ignored in user code

       Sometimes, you want to give a particular field or type a larger alignment than it technically requires.
       You can do that using the attribute #[repr(align(n))].
 */


#[repr(C)]  //As C language, the compiler will not reorder the fields
//#repr(Rust)]
//#[repr(packed)] // This may lead to much slower code
struct Foo {
    tiny: bool,  //1 bit
    normal: u32,  //4-byte type
    small: u8,    //1-byte value
    long: u64,
    short: u16,
}