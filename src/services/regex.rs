use regex::Regex;

pub struct UsernameRegex {
    pub length: Regex,
    pub uppercase: Regex,
    pub digit: Regex,
    pub special: Regex,
}

pub struct RegexService {
    pub email_regex: Regex,
    pub username_regex: UsernameRegex,
}

impl UsernameRegex {
    pub fn new(
        username_regex_vec: Vec<Regex>
    ) -> Self {
        let length = username_regex_vec[0].clone();
        let uppercase = username_regex_vec[1].clone();
        let digit = username_regex_vec[2].clone();
        let special = username_regex_vec[3].clone();
        Self {
            length,
            uppercase,
            digit,
            special
        }
    }
}

impl RegexService {
    pub fn new(
        email_regex: Regex,
        username_regex_vec: Vec<Regex>,
    ) -> Self{
        Self {
            email_regex,
            username_regex: UsernameRegex::new(username_regex_vec)
        }
    }
}
