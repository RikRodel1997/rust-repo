pub struct FloatStruct {
    pub result: f64,
}

impl FloatStruct {
    pub fn new(input: f64) -> Self {
        if input < 0.0 {
            panic!("input {input} should be equal than or greater than 0");
        }
        Self {
            result: Self::match_input(input),
        }
    }

    fn match_input(input: f64) -> f64 {
        match input {
            0.0 => input * 2.0,
            1.0 => input * 2.0,
            3.0 => input * 2.0,
            6.0 => input * 2.0,
            _ => input * 1.5,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_doubles() {
        let inputs = vec![0.0, 1.0, 3.0, 6.0];
        for input in inputs.iter() {
            let result = FloatStruct::new(*input);
            assert_eq!(result.result, input * 2.0)
        }
    }

    #[test]
    fn test_input_times_one_point_five() {
        let inputs = vec![2.0, 4.0, 10.0];
        for input in inputs.iter() {
            let result = FloatStruct::new(*input);
            assert_eq!(result.result, input * 1.5)
        }
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        FloatStruct::new(-2.0);
    }
}
