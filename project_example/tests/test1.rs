use project_example::sub::file2::*;
use project_example::sub::submod::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let res = add(1, 2);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = subtract(10, 12);
        assert_eq!(res, -2);
    }

    #[test]
    fn fail_test() {
        let res = subtract(10, 12);
        assert_eq!(res, -3);
    }
}