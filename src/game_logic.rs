use ratatui::style::Color;

#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    Yellow,
    Green,
    Nothing,
}

impl Status {
    pub fn color(&self) -> Color {
        match self {
            Status::Green => {Color::Green}
            Status::Yellow => {Color::Yellow}
            Status::Nothing => {Color::White}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub c: char,
    pub status: Status,
}

impl Element {
    pub fn default() -> Self {
        Self { c: '1', status: Status::Nothing }
    }
}


pub fn check_word(word: String, solution: String) -> Vec<Element> {
    
    let mut return_word = vec![Element::default(); 5];
    for (i, c) in word.chars().enumerate() {
        if c == solution.chars().nth(i).unwrap() {
            return_word[i] = Element {
                c,
                status: Status::Green,
            };
        }
    }

    for (i, c) in word.chars().enumerate() {
        if return_word[i].status == Status::Green {
            continue;
        }
        return_word[i] = if solution.contains(c) {
            Element {
                c,
                status: Status::Yellow,
            }
        } else {
            Element {
                c,
                status: Status::Nothing,
            }
        }
    }
    return_word
}
