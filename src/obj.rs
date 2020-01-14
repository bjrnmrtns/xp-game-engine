pub fn lex<T, F>(input: T, mut callback: F)
    where T: std::io::BufRead, F: std::ops::FnMut(&str, &[&str])
{
    let mut ml = String::new();
    for line in input.lines() {
        // Get line and remove everything after #.
        let line_with_comments = line.unwrap();
        let line = line_with_comments.split("#").next().unwrap();

        if line.ends_with('\\') {
            ml.push_str(&line[0..line.len()-1]);
            ml.push(' ');
            continue
        }
        ml.push_str(line);

        let mut tokens = ml.split_whitespace();
        if let Some(statement) = tokens.next() {
            let mut args = Vec::new();
            for token in tokens {
                args.push(token);
            }
            callback(&statement, &args);
        }
        ml.clear();
    }
}