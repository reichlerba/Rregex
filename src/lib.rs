// (C) Benjamin Reichler

pub mod rregex;

#[cfg(test)]
mod tests {
    use super::*;
    use rregex::Rregex;

    fn assert_rregex_recognizes_finite(e: Rregex, strs_to_recognize: Vec<&str>) {

        // Check empty string
        let recognizes = e.recognizes("");
        assert!(recognizes == strs_to_recognize.contains(&""));

        // Check all combinations of 1 length strings.
        for c in ' '..='~' {
            let s = c.to_string();
            let expected = strs_to_recognize.contains(&s.as_str());
            let recognizes = e.recognizes(&s);

            assert_eq!(
                recognizes,
                expected,
                "Unexpected recognition result for {:?}",
                s
            );
        }

        // Check all combinations of 2 length strings.
        for a in ' '..='~' {
            for b in ' '..='~' {
                let s = format!("{}{}", a, b);
                let expected = strs_to_recognize.contains(&s.as_str());
                let recognizes = e.recognizes(&s);

                assert_eq!(
                    recognizes,
                    expected,
                    "Unexpected recognition result for {:?}",
                    s
                );
            }
        }
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
        assert!(false);
    }

}
