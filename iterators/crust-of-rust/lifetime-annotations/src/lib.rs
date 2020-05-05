
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.remainder.find(self.delimiter);
        println!("{:?}", x);
        // println!("{:#?}", x);
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            println!("next called. got: {}", until_delimiter);
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            println!("self.remainder.is_empty()");
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
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
    assert!(letters.eq(vec!["a", "b", "c", "d"].into_iter()));


    let git = "Github";
    for (i, c) in git.char_indices() {
        println!("{} -> {}", i, c);
    }
}
