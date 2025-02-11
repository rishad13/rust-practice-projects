struct StrSpliterator<'a> {
    text: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSpliterator<'a> {
    fn new(text: &'a str, delimiter: &'a str) -> Self {
        StrSpliterator { text, delimiter }
    }
}

//implementing iterator, so that we can use iterate over the items in for loop
impl<'a> Iterator for StrSpliterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.text.is_empty() {
            None
        } else if self.text.contains(self.delimiter) {
            let index = self.text.find(self.delimiter);
            if let Some(index) = index {
                let result = &self.text[..index];
                self.text = &self.text[index + self.delimiter.len()..];
                Some(result)
            } else {
                panic!("weird, delimiter contains in text, but couldn't  find the position.");
            }
        } else {
            let result = self.text;
            self.text = "";
            Some(result)
        }
    }
}

fn print_words_in_between_delimiters(str_spliterator: StrSpliterator) {
    for word in str_spliterator {
        println!("{}", word);
    }
}

fn take_ownership_of_text(_: String) {
    println!("I took ownership of and dropped it, Haha!");
}

fn main() {
    let text = String::from("Hello I'm just learning Rust, I want to learn Rust deeply, But Rust have many concept which I haven't even heard yet, hopefully Rust will make a better developer");
    let delimiter = String::from("Rust");
    let str_spliterator = StrSpliterator::new(&text, &delimiter);
   // take_ownership_of_text(text); //if enabled this line, won't able to compile, since the text's lifetime have over.
    print_words_in_between_delimiters(str_spliterator);
}
