#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    YELLOW,
    GREEN,
    NONE,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub c: char,
    pub status: Status,
}

impl Element {
    pub fn default() -> Self {
        Self { c: '1', status: Status::NONE }
    }
}


pub fn check_word(word: String, solution: String) -> Vec<Element> {
    
    let mut return_word = vec![Element::default(); 5];;
    for (i, c) in word.chars().enumerate() {
        if c == solution.chars().nth(i).unwrap() {
            return_word[i] = Element {
                c,
                status: Status::GREEN,
            };
        }
    }

    for (i, c) in word.chars().enumerate() {
        if return_word[i].status == Status::GREEN {
            continue;
        }
        return_word[i] = if solution.contains(c) {
            Element {
                c,
                status: Status::YELLOW,
            }
        } else {
            Element {
                c,
                status: Status::NONE,
            }
        }
    }
    return_word
}
