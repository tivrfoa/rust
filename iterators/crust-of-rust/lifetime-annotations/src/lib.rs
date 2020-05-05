
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
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

#[test]
fn it_works() {
    let haystack = "a b c d e";

    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    
    // The 2 lines above are equivalent to:
    let letters2: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters2, vec!["a", "b", "c", "d", "e"]);

    //for l in letters {
    //    println!("{}", l);
    //}
}

#[test]
fn tail() {
    let haystack = "a b c d ";

    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));


    let git = "Github";
    for (i, c) in git.char_indices() {
        println!("{} -> {}", i, c);
    }
}

#[test]
fn empty_tail() {
    let haystack = "a b c d";

    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));
}
