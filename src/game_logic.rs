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
            Status::Green => Color::Green,
            Status::Yellow => Color::Yellow,
            Status::Nothing => Color::White,
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
        Self {
            c: '1',
            status: Status::Nothing,
        }
    }
}

pub fn check_word(word: String, solution: String) -> Vec<Element> {
    let mut return_elements = vec![Element::default(); 5];
    for (i, c) in word.chars().enumerate() {
        if c == solution.chars().nth(i).unwrap() {
            return_elements[i] = Element {
                c,
                status: Status::Green,
            };
        }
    }

    for (i, c) in word.chars().enumerate() {
        if return_elements[i].status == Status::Green {
            continue;
        }

        return_elements[i] = if yellow_or_not(c, &return_elements, &solution) {
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

    return_elements
}

fn yellow_or_not(c: char, return_elements: &Vec<Element>, solution: &str) -> bool {
    let mut exists_already = false;
    for element in return_elements {
        if element.c == c {
            exists_already = true;
            break;
        }
    }

    !(solution.contains(c) && exists_already)
}
