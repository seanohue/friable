#[cfg(test)]

pub mod friable {

    fn equals(a: &isize, b: &isize) -> bool {
        a == b
    }

    fn is<T>(a: &T, b: &T) -> bool {
        a as *const T == b as *const T
    }


    #[test]
    fn equality() {
        let i = 1;
        let j = 1;
        let expected = true;
        let actual = equals(&i, &j);
        assert!(actual, expected);
    }

    #[test]
    fn inequality() {
        let i = 1;
        let j = 77;
        let actual = equals(&i, &j);
        let expected = false;
        assert!(actual, expected);
    }

}
