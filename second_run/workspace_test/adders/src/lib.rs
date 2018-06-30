//! # adders Crate
//! This crate contains just one function for this example project on workspaces, but that function
//! has some interesting behaviour…  
//! See the documentation of add_one_int()!


extern crate num;
use num::*;

/// # add_one_int()
/// A function, that takes any **Integer-Type** and returns it increased by 1.
///
/// ## Example
/// ```
/// assert_eq!(add_one_int(9823), 9824);
/// ```
///
/// ## Quick Explanation of how this works
/// Generic Functions for Numeric Types are quite a nuisance to implement.
///
/// A Numeric Literal like `42i32` can't be *generic* over any types, including other Numeric
/// Types.  
/// That means the following won't work, because `1` is not of type `T`:
/// ```
/// fn add_one_int<T> (n: T) -> T {
///     n+1
/// }
/// ```
/// In this case `1` is of the standard integer type `i32`.
///
/// To use Numeric Types in Generic Functions, like we want to here, you have to reconstruct the
/// basic values of the desired base (in this case *decimal*) using Generic Types.  
/// The traits used to make such Types are defined in the `Num`-crate as `num::One` and `num::Zero`.
/// With those two types you can make any number in any base and use them to do the Generic Addition.
///
/// It follows that the following works:
/// ```
/// extern crate num;
/// use num::*;
///
/// fn add_one_int<T: Integer + Copy> (n: T) -> T {
///     let _1 = T::one();
///     n + _1
/// }
/// ```
/// First I define `_1` as `T::one()`. Since `T` is required to have the Integer trait, it has an
/// implementation for the `num`-crate's `One::one()` method. That method returns a "multiplicative
/// identity of Self and 1". That means the following two laws apply to any operation, which
/// employs `_1`:
/// ```
/// a  * _1 = a
/// _1 * a  = a
/// ```
/// So `_1` now holds a Generic Type with the value of 1!!!   
/// We can use it to construct any values required for different bases and operations!
///
/// For the `add_one_int()` Function we need only `_1`, cause we only want to add 1.  
/// If we were to implement, say `add_15_int()`, the code might look something like this:
/// ```
/// extern crate num;
/// use num::*;
///
/// fn add_15_int<T: Integer + Copy> (n: T) -> T {
///
///     let _1 = T::one();
///     let _2 = _1 + _1;
///     let _3 = _2 + _1;
///     let _5 = _3 + _1;
///     let _10 = _5 + _5;
///     let _15 = _10 + _5;
///
///     n + _15
/// }
/// ```
/// **This structure can be used analogously for any Generically Typed operation with integers.**
///
/// **Sidenote:** I snuck in a trait bound for the `Copy` trait in the Function signatures up
/// above.  
/// `T` needs the `Copy` trait, so it will behave like any non-Generic Numeric Type, all of which are
/// primitives and as such are automatically duplicated, so we don't have a hassle with references.
/// If I didn't do this, I couldn't refer to `_1` to build `_2`, etc.  
/// `Clone` is actually a trait bound in the impl definitions for `One` and `Zero` in `Num`, `Copy`
/// is a more specific sub-trait of `Clone` used for the numeric primitives, so of
/// course we need to make sure, that any value for `T` is going to have these bounds as well.
///
/// This is actually a good general rule: **If Impl's on library-defined types you use have
/// trait bounds, refer to them for the trait bounds in your Generic definitions.** The type you use
/// almost certainly needs to be compatible with the type from the library, and as such needs all
/// the same bounds. Often you could actually use the library type directly, instead of the trait
/// bounds, here specifically not, though.
// An add_one_int Function, that contains way more code, than it needs (for illustrative purposes)
pub fn add_one_int<T> (n: T) -> T
    where T: Integer + Copy
{
    let _0 = T::zero();     // not required for function
    let _1 = T::one();
    let _2 = _1 + _1;       // not required for function
    let _3 = _1 + _2;       //  "     "      "     "
    let _4 = _2 + _2;       //  "     "      "     "
    let _5 = _3 + _2;       //  "     "      "     "
    let _6 = _3 + _3;       //  "     "      "     "
    let _7 = _4 + _3;       //  "     "      "     "
    let _8 = _4 + _4;       //  "     "      "     "
    let _9 = _5 + _4;       //  "     "      "     "
    let _10 = _5 + _5;      //  "     "      "     "

    n + _1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn adds_one() {
        // Test add_one_int for each Integer Type
        assert!(add_one_int(42u8) == 43u8, "Adding to a u8-Type tested!");
        assert!(add_one_int(42u16) == 43u16, "Adding to a u16-Type tested!");
        assert!(add_one_int(42u32) == 43u32, "Adding to a u32-Type tested!");
        assert!(add_one_int(42u64) == 43u64, "Adding to a u64-Type tested!");
        assert!(add_one_int(-42i8) == -41i8, "Adding to a i8-Type tested!");
        assert!(add_one_int(-42i16) == -41i16, "Adding to a i16-Type tested!");
        assert!(add_one_int(-42i32) == -41i32, "Adding to a i32-Type tested!");
        assert!(add_one_int(-42i64) == -41i64, "Adding to a i64-Type tested!");

        // Test at least one basic edge case
        assert_eq!(add_one_int(0), 1);
    }

}
