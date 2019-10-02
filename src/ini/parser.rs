#[derive(Clone, Debug)]
enum ParseEvent<'i> {
    // These fuck up everything and I'm not sure how.
    // Section(&'i str),
    Property { key: &'i str, value: &'i str },
}

pub struct ParseEventIter<'i> {
    step: u8,
    event: ParseEvent<'i>,
}

impl<'i> Iterator for ParseEventIter<'i> {
    type Item = &'i str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step {
            0 => match self.event {
                // ParseEvent::Section(s) => {
                //     self.step += 1;
                //     Some(s)
                // }
                ParseEvent::Property { key, .. } => {
                    self.step += 1;
                    Some(key)
                }
            },

            1 => match self.event {
                // ParseEvent::Section(_) => None,
                ParseEvent::Property { value, .. } => {
                    self.step += 1;
                    Some(value)
                }
            },

            _ => None,
        }
    }
}

impl<'i> IntoIterator for ParseEvent<'i> {
    type IntoIter = ParseEventIter<'i>;
    type Item = &'i str;

    fn into_iter(self) -> Self::IntoIter {
        ParseEventIter {
            step: 0,
            event: self,
        }
    }
}

pub fn parse_from_str(s: &str) -> impl Iterator<Item = &str> {
    s.trim()
        .lines()
        .filter_map(|line| match line.trim() {
            // Comments
            line if line.starts_with("//") || line.is_empty() => None,

            // Sections
            // line if line.starts_with('[') && line.ends_with(']') => {
            //     Some(ParseEvent::Section(&line[1..(line.len() - 1)]))
            // }

            // Properties
            line => {
                if let Some(boundary) = line.find('=') {
                    let (key, value) = line.split_at(boundary);
                    Some(ParseEvent::Property {
                        key,
                        value: value.trim_start_matches('='),
                    })
                } else {
                    None
                }
            }
        })
        .flatten()
}
