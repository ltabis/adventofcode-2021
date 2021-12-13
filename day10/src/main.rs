use std::str::Chars;

fn main() {
    let input = load_input("./src/input.txt").unwrap();

    println!("ex01: {}", compute_score(&input));
}

#[derive(Default, Debug, Clone)]
struct Chunk {
    start: char,
    end: char,
    children: Vec<Chunk>,
}

impl Chunk {
    fn new(start: char, end: char) -> Self {
        Self {
            start,
            end,
            children: Vec::default(),
        }
    }
}

#[derive(Debug)]
pub enum AnalyserStatus {
    Incomplete(char),
    Corrupted(char),
    Acceptable(char),
}

#[derive(Default, Debug)]
struct Analyser {
    types: Vec<Chunk>,
    chunks: Vec<Chunk>,
    source: String,
}

fn compute_score(input: &str) -> usize {
    let mut score = 0;

    for line in input.lines() {
        let mut analyser = AnalyserBuilder::new()
            .source(line)
            .with_chunk('(', ')')
            .with_chunk('[', ']')
            .with_chunk('<', '>')
            .with_chunk('{', '}')
            .build();

        score += if let AnalyserStatus::Corrupted(character) = analyser.analyse() {
            match character {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        } else {
            0
        };
    }

    score
}

impl Analyser {
    fn analyse(&mut self) -> AnalyserStatus {
        let (chunks, status) = self.create_chunk_tree(&mut self.source.chars());
        self.chunks = chunks;
        status
    }

    fn try_repair(&mut self) -> Option<String> {
        if self.chunks.is_empty() {
            return None;
        }

        None
    }

    fn create_chunk_tree(&self, source: &mut Chars) -> (Vec<Chunk>, AnalyserStatus) {
        let mut chunks = Vec::default();

        loop {
            let character = match source.next() {
                Some(c) => c,
                None => return (chunks, AnalyserStatus::Incomplete(char::default())),
            };

            // a new block is starting.
            if let Some(chunk_type) = self.types.iter().find(|t| t.start == character) {
                let mut new_chunk = chunk_type.clone();
                let (children, status) = self.create_chunk_tree(source);
                new_chunk.children.extend(children);
                chunks.push(new_chunk.clone());

                match status {
                    AnalyserStatus::Acceptable(c) => {
                        // a block is closing
                        if chunks.last().unwrap().end == c {
                            continue;
                        }
                        // corrupted, the current chunk isn't closed.
                        else {
                            return (chunks, AnalyserStatus::Corrupted(c));
                        }
                    }
                    _ => return (chunks, status),
                }
            }
            // the closing character could be from the parent.
            else {
                return (chunks, AnalyserStatus::Acceptable(character));
            }
        }
    }
}

struct AnalyserBuilder {
    types: Vec<Chunk>,
    source: String,
}

impl AnalyserBuilder {
    fn new() -> Self {
        Self {
            types: Vec::default(),
            source: String::default(),
        }
    }

    fn source(mut self, src: &str) -> Self {
        self.source = src.to_string();
        self
    }

    fn with_chunk(mut self, start: char, end: char) -> Self {
        self.types.push(Chunk::new(start, end));
        self
    }

    fn build(self) -> Analyser {
        Analyser {
            types: self.types,
            chunks: Vec::default(),
            source: self.source,
        }
    }
}

fn load_input(src: &str) -> std::io::Result<String> {
    std::fs::read_to_string(src)
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_example() {
        let input = load_input("./src/example.txt").expect("couldn't read example.txt");

        for line in input.lines() {
            let mut analyser = AnalyserBuilder::new()
                .source(line)
                .with_chunk('(', ')')
                .with_chunk('[', ']')
                .with_chunk('<', '>')
                .with_chunk('{', '}')
                .build();

            println!("{:?}", analyser.analyse());
            // println!("{:#?}", analyser.chunks);
        }
    }
}
