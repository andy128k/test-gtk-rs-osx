fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[gtk4::test]
    fn test_label() {
        let l = gtk4::Label::builder().build();
    }

    #[test]
    fn test_math() {
        assert_eq!(2 + 2, 4);
    }
}
