# vecfold: Fold vectors of Results into Result of vector

This package provide a generic way to _"unwrap"_ a `Vec<Result<T,E>>` into
`Result<Vec<&T>, &E>`. Here is an example:

```rust
let valid: Vec<Result<u32, _>> = "1,2,3".split(",").map(|x| x.parse::<u32>()).collect();
let invalid: Vec<Result<u32, _>> = "1,2,a".split(",").map(|x| x.parse::<u32>()).collect();

// happy path, no errors, just the values
assert_eq!(vec![&1, &2, &3], valid.foldr().unwrap());

// sad path returns the error
assert!(invalid.foldr().is_err());
```

If you need to collect all errors you can use `.foldr_bisect`. It converts
`Vec<Result<T, E>>`, to `(Vec<&T>, Vec<&E>)`.

```rust
// happy path, no errors, return empty error vector
assert_eq!((vec![&1, &2, &3], vec![]), valid.foldr_bisect());

// sad path, populate error vector
let (ok, _) = invalid.foldr_bisect();
assert_eq!(vec![&1, &2], ok);
```
