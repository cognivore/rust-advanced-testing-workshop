#[cfg(test)]
mod tests {
    #[test]
    fn intro() {
        let msg = "I've installed `insta`!";
        assert_eq!(msg, "I've installed `insta`!")
    }
}
