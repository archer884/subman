#[derive(Clone, Debug)]
enum ParseEvent<'i> {
    Section(&'i str),
    Property {
        key: &'i str,
        value: &'i str,
    }
}

pub enum ParseEventIter<'i> {
    SectionIter(u8, &'i str),
    PropertyIter(u8, &'i str, &'i str),
}

impl ParseEventIter<'_> {
    fn step(&mut self) {
        match self {
            ParseEventIter::SectionIter(ref mut step, _) => *step += 1,
            ParseEventIter::PropertyIter(ref mut step, ..) => *step += 1,
        }
    }
}

impl<'i> Iterator for ParseEventIter<'i> {
    type Item = &'i str;

    fn next(&mut self) -> Option<Self::Item> {
        self.step();
        match self {
            ParseEventIter::SectionIter(1, s) => Some(s),
            ParseEventIter::PropertyIter(1, key, _) => Some(key),
            ParseEventIter::PropertyIter(2, _, value) => Some(value),

            _ => None,
        }
    }
}

impl<'i> IntoIterator for ParseEvent<'i> {
    type IntoIter = ParseEventIter<'i>;
    type Item = &'i str;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ParseEvent::Section(section) => ParseEventIter::SectionIter(0, section),
            ParseEvent::Property { key, value } => ParseEventIter::PropertyIter(0, key, value),
        }
    }
}

pub fn parse_from_str(s: &str) -> impl Iterator<Item = &str> {
    s.lines().filter_map(|line| match line.trim() {
        line if line.starts_with('[') && line.ends_with(']') => Some(ParseEvent::Section(&line[1..(line.len() - 2)])),
        line if line.starts_with("//") || line.is_empty() => None,
        line => {
            if let Some(boundary) = line.find('=') {
                Some(ParseEvent::Property {
                    key: &line[..boundary],
                    value: &line[(boundary + 1)..],
                })
            } else {
                None
            }
        }
    }).flatten()
}
