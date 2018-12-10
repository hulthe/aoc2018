use std::str::FromStr;
use std::error::Error;
use std::fmt::{Display, Formatter, self};
use std::option::NoneError;
use std::num::ParseIntError;

pub struct TreeNode {
    pub children: Vec<TreeNode>,
    pub metadata: Vec<usize>,
}


#[derive(Debug)]
pub enum TreeParseError {
    UnexpectedEndOfInput,
    ParseError(ParseIntError),
}
impl Error for TreeParseError {}

impl From<NoneError> for TreeParseError {
    fn from(_: NoneError) -> TreeParseError {
        TreeParseError::UnexpectedEndOfInput
    }
}

impl From<ParseIntError> for TreeParseError {
    fn from(e: ParseIntError) -> TreeParseError {
        TreeParseError::ParseError(e)
    }
}

impl Display for TreeParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TreeParseError::UnexpectedEndOfInput => "Unexpected end of input".into(),
            TreeParseError::ParseError(e) => format!("Could not parse data: {}", e),
        })
    }
}


impl TreeNode {
    fn rec_parse<'a, 'b, I>(iter: &'a mut I) -> Result<TreeNode, TreeParseError>
        where I: Iterator<Item=&'b str>,
              'b: 'a,
    {
        let child_count: usize = iter.next()?.parse()?;
        let meta_count: usize = iter.next()?.parse()?;
        let children: Vec<TreeNode> = (0..child_count)
            .map(|_| Self::rec_parse(iter))
            .collect::<Result<_, TreeParseError>>()?;
        let metadata: Vec<usize> = (0..meta_count)
            .map(|_| iter.next().map(|s| s.parse()))
            .collect::<Option<Result<_, ParseIntError>>>()??;

        Ok(TreeNode {
            children,
            metadata,
        })
    }

    pub fn iter(&self) -> TreeIter {
        TreeIter {
            node: &self,
            probe: None,
            state: 0,
        }
    }
}

impl FromStr for TreeNode {
    type Err = TreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::rec_parse(&mut s.trim().split(' '))
    }
}

pub struct TreeIter<'a> {
    node: &'a TreeNode,
    probe: Option<Box<TreeIter<'a>>>,
    state: usize,
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = &'a Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rn = None;
        if let Some(i) = &mut self.probe {
            rn = i.next();
        };

        if let Some(n) = rn {
            Some(n)
        } else if self.state < self.node.children.len() {
            self.probe = Some(Box::new(TreeIter{
                node: &self.node.children[self.state],
                probe: None,
                state: 0,
            }));
            self.state += 1;
            self.probe.as_mut().unwrap().next()
        } else if self.state == self.node.children.len() {
            self.state += 1;
            Some(&self.node.metadata)
        } else {
            None
        }
    }
}
