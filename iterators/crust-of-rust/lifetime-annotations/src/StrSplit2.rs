
#[derive(Debug)]
pub struct StrSplit2<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'haystack, 'delimiter> StrSplit2<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit2<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                println!("next called. got: {}", until_delimiter);
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_char2(s: &str, c: char) -> &str {
    StrSplit2::new(s, &format!("{}", c))
        .next()
        .expect("one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters = StrSplit2::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    
    // The 2 lines above are equivalent to:
    let letters2: Vec<_> = StrSplit2::new(haystack, " ").collect();
    assert_eq!(letters2, vec!["a", "b", "c", "d", "e"]);

    //for l in letters {
    //    println!("{}", l);
    //}
}

#[test]
fn tail() {
    let haystack = "a b c d ";

    let letters = StrSplit2::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));


    let git = "Github";
    for (i, c) in git.char_indices() {
        println!("{} -> {}", i, c);
    }
}

#[test]
fn empty_tail() {
    let haystack = "a b c d";

    let letters = StrSplit2::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));
}
