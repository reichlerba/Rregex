// (C) Benjamin Reichler

pub mod rregex;

#[cfg(test)]
mod tests {
    use super::*;
    use rregex::Rregex;

    fn assert_rregex_recognizes_finite(e: Rregex, strs_to_recognize: Vec<&str>) {

        // Each call to assert_rregex_recognizes_finite() takes on the order of
        // O((MAX_CHAR - MIN_CHAR) ^ MAX_LENGTH_TO_CHECK) time, so keep small.
        const MAX_LENGTH_TO_CHECK: usize = 3;

        fn check_strings_max_length(e: Rregex, max_length: usize, strs_to_recognize: Vec<&str>) {

            // For full range of printable ASCII, use b' ' to b'~'.
            const MIN_CHAR: u8 = b'a';
            const MAX_CHAR: u8 = b'f';

            // Check strings of exactly length max_length.

            // Keep track of a buffer of length max_length to hold the string being checked.
            // Initially string is all MIN_CHAR.
            let mut string_buf = vec![MIN_CHAR; max_length];

            // Change each element in each spot of string_buf and check.
            'outer: loop {
                // The unwrap() used is ok only because this is a test, 
                // and also we know string is guaranteed valid.
                let string = String::from_utf8(string_buf.clone()).unwrap();

                // Conduct test.
                assert_eq!(
                    e.recognizes(&string),
                    strs_to_recognize.contains(&string.as_str()),
                    "Unexpected recognition result for input {:?}", string
                );

                // Check base case: max_length is the minimum value possible.
                // Base case needs to be checked after testing for empty string,
                // but within 'outer so we can still break out.
                if max_length == 0 {
                    return;
                }

                for i in (0..max_length).rev() {
                    if string_buf[i] < MAX_CHAR {
                        string_buf[i] += 1;
                        break;
                    } else {
                        string_buf[i] = MIN_CHAR;
                        if i == 0 {
                            // We have exhausted all combinations of length max_length.
                            break 'outer;
                        }
                    }
                }
            }

            // Check all shorter strings recursively.
            check_strings_max_length(e, max_length - 1, strs_to_recognize);
        }

        check_strings_max_length(e, MAX_LENGTH_TO_CHECK, strs_to_recognize);

    }

    #[test]
    fn check_empty_set_recognizing() {
        // Empty set should not recognize anything.
        let e = Rregex::get_empty_set();
        assert_rregex_recognizes_finite(e, vec![]);
    }

    #[test]
    fn check_epsilon_recognizing() {
        // {\epsilon} should only recognize empty string.
        let e = Rregex::get_epsilon();
        assert_rregex_recognizes_finite(e, vec![""]);
    }

    #[test]
    fn check_a_recognizing() {
        // {'a'} should only recognize "a".
        let e = Rregex::get_symbol('a');
        assert_rregex_recognizes_finite(e, vec!["a"]);
    }

    #[test]
    fn check_union_recognizing() {
        
        // {} U {} should recognize nothing.
        let empty_1 = Rregex::get_empty_set();
        let empty_2 = Rregex::get_empty_set();
        let e = Rregex::union(empty_1, empty_2);
        assert_rregex_recognizes_finite(e, vec![]);

        // {} U {\epsilon} and {\epsilon} U {} should recognize empty string.
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_empty_set(), 
                Rregex::get_epsilon()
            ), 
            vec![""]
        );
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_epsilon(), 
                Rregex::get_empty_set()
            ), 
            vec![""]
        );

        // {} U {'a'} and {'a'} U {} should only recognize "a".
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_empty_set(), 
                Rregex::get_symbol('a')
            ), 
            vec!["a"]
        );
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_symbol('a'),
                Rregex::get_empty_set() 
            ), 
            vec!["a"]
        );

        // {\epsilon} U {\epsilon} should only recognize empty string.
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_epsilon(),
                Rregex::get_epsilon() 
            ), 
            vec![""]
        );

        // {\epsilon} U {'a'} and {'a'} U {\epsilon} should recognize "a" and empty string.
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_epsilon(),
                Rregex::get_symbol('a') 
            ), 
            vec!["", "a"]
        );
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_symbol('a') ,
                Rregex::get_epsilon()
            ), 
            vec!["", "a"]
        );

        // {'a'} U {'a'} should only recognize "a".
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_symbol('a'),
                Rregex::get_symbol('a')
            ),
            vec!["a"]
        );

        // {'a'} U {'b'} should recognize "a" and "b".
        assert_rregex_recognizes_finite(
            Rregex::union(
                Rregex::get_symbol('a'),
                Rregex::get_symbol('b')
            ),
            vec!["a", "b"]
        );
        
        
    }

    #[test]
    fn check_concat_recognizing() {

        // {} . {} should recognize nothing.
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_empty_set(),
                Rregex::get_empty_set()
            ),
            vec![]
        );

        // {} . {\epsilon} and {\epsilon} . {} should recognize nothing.
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_empty_set(),
                Rregex::get_epsilon()
            ),
            vec![]
        );
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_epsilon(),
                Rregex::get_empty_set()
            ),
            vec![]
        );

        // {} . {'a'} and {'a'} . {} should recognize nothing.
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_empty_set(),
                Rregex::get_symbol('a')
            ),
            vec![]
        );
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_symbol('a'),
                Rregex::get_empty_set()
            ),
            vec![]
        );

        // {\epsilon} . {\epsilon} should recognize empty string.
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_epsilon(),
                Rregex::get_epsilon()
            ),
            vec![""]
        );

        // {\epsilon} . {'a'} and {'a'} . {\epsilon} should recognize "a".
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_epsilon(),
                Rregex::get_symbol('a')
            ),
            vec!["a"]
        );
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_symbol('a'),
                Rregex::get_epsilon()
            ),
            vec!["a"]
        );

        // {'a'} . {'a'} should recognize "aa".
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::get_symbol('a'),
                Rregex::get_symbol('a')
            ),
            vec!["aa"]
        );

        // {'a'} . {'b'} should recognize "ab".
        assert_rregex_recognizes_finite(    
            Rregex::concat(
                Rregex::get_symbol('a'),
                Rregex::get_symbol('b')
            ),
            vec!["ab"]
        );

        // {'a'} . {'b'} . {'c'} should recognize "abc".
        assert_rregex_recognizes_finite(
            Rregex::concat(
                Rregex::concat(
                    Rregex::get_symbol('a'),
                    Rregex::get_symbol('b')
                ),
                Rregex::get_symbol('c')
            ),
            vec!["abc"]
        );
    }

}
