fn main() {
    for i in 1..=5 {
        // Use string repetition and formatting (placeholder)
        let line = format!("{:}", i.to_string().repeat(i));
        println!("{}", line);
    }
}
