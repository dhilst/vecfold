use std::error::Error;
use std::fmt::Debug;

pub trait VecFoldResult<'a, T, E>
where
    T: Debug,
    E: Error + Debug + 'a,
{
    fn foldr(&self) -> Result<Vec<&T>, &E>;
}

/// Converts Vec<Result<T, E>> -> Result<Vec<&T>, &E> stopping on first error
///
/// If no error is found Vec<&T> foldr is equivalent of Ok(v.map(|x| x.unwrap()))
/// If an error is found, the first error is returned and the vector is not walked further
impl<'a, T, E> VecFoldResult<'a, T, E> for Vec<Result<T, E>>
where
    T: Debug,
    E: Error + Debug + 'a,
{
    fn foldr(&self) -> Result<Vec<&T>, &E> {
        let mut buf = Vec::new();
        for i in self {
            if let Err(err) = i {
                return Err(err);
            } else {
                buf.push(i.as_ref().unwrap());
            }
        }

        return Ok(buf);
    }
}

pub trait VecFoldResultBisect<T, E>
where
    T: Debug,
    E: Error + Debug,
{
    fn foldr_bisect(&self) -> (Vec<&T>, Vec<&E>);
}

/// Converts Vec<Result<T, E>> -> (Vec<&T>, Vec<&E>)
///
/// All Ok values are collected on left side of tuple
/// All Err vaues are collected on right side of tuple
impl<T, E> VecFoldResultBisect<T, E> for Vec<Result<T, E>>
where
    T: Debug,
    E: Error + Debug,
{
    fn foldr_bisect(&self) -> (Vec<&T>, Vec<&E>) {
        let mut res = Vec::new();
        let mut err = Vec::new();
        for i in self {
            match i {
                Ok(i) => res.push(i),
                Err(e) => err.push(e),
            }
        }

        return (res, err);
    }
}

mod test {
    use super::*;
    #[derive(Debug, PartialEq)]
    struct StaticStringError(&'static str);
    use std::fmt;

    impl fmt::Display for StaticStringError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for StaticStringError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    #[test]
    fn test() {
        use super::*;
        let r: Vec<Result<_, StaticStringError>> = vec![Ok(1), Ok(2), Ok(3)];
        assert_eq!(vec![&1, &2, &3], r.foldr().unwrap());
        assert_eq!((vec![&1, &2, &3], vec![]), r.foldr_bisect());

        let r: Vec<Result<_, StaticStringError>> =
            vec![Ok(1), Err(StaticStringError("oops")), Ok(3)];
        assert_eq!(&StaticStringError("oops"), r.foldr().unwrap_err());
        assert_eq!(
            (vec![&1, &3], vec![&StaticStringError("oops")]),
            r.foldr_bisect()
        );
    }

    #[test]
    fn test_with_parse() {
        let valid: Vec<Result<u32, _>> = "1,2,3".split(",").map(|x| x.parse::<u32>()).collect();

        let invalid: Vec<Result<u32, _>> = "1,2,a".split(",").map(|x| x.parse::<u32>()).collect();

        // happy path, no errors, just the values
        assert_eq!(vec![&1, &2, &3], valid.foldr().unwrap());

        // sad path returns the error
        assert!(invalid.foldr().is_err());

        // happy path, no errors, return empty error vector
        assert_eq!((vec![&1, &2, &3], vec![]), valid.foldr_bisect());

        // sad path, populate error vector
        let (ok, _) = invalid.foldr_bisect();
        assert_eq!(vec![&1, &2], ok);
    }
}
