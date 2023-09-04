// structs1.rs
//
// Address all the TODOs to make the tests pass!

// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}



// Rust has more than one type of struct. Three actually, all variants are used to package related data together.
// There are normal (or classic) structs. These are named collections of related data stored in fields.
// Tuple structs are basically just named tuples.
// Finally, Unit-like structs. These don't have any fields and are useful for generics.

// In this exercise you need to complete and implement one of each kind.
// Read more about structs -
// The Book: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// Rust by Example : https://doc.rust-lang.org/rust-by-example/custom_types/structs.html 
