fn main() {
    // You can optionally experiment here.
        let some_value = Some(42);
        let no_value: Option<i32> = None;
    
        if let Some(value) = some_value {
            println!("Found a value: {}", value);
        } else {
            println!("No value found!");
        }
    
        if let Some(value) = no_value {
            println!("Found a value: {}", value);
        } else {
            println!("No value found!");
        }
    
        // Using while-let with nested Option
        let mut values = vec![None, Some(1), Some(2), Some(3)];
    
        while let Some(Some(value)) = values.pop() {
            println!("Popped value: {}", value);
        }    
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}