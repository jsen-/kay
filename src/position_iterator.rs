use std::io::Result;

pub struct PositionIterator<T>
where
    T: Iterator<Item = Result<char>>,
{
    it: T,
    index: usize,
    line: usize,
    col: usize,
}

impl<T> Iterator for PositionIterator<T>
where
    T: Iterator<Item = Result<char>>,
{
    type Item = Result<char>;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|ch| {
            ch.map(|ch| {
                if ch == '\n' {
                    self.col = 0;
                    self.line += 1;
                }
                self.col += 1;
                self.index += 1;
                ch
            })
        })
    }
}

impl<T> PositionIterator<T>
where
    T: Iterator<Item = Result<char>>,
{
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn col(&self) -> usize {
        self.col
    }
    // pub fn index(&self) -> usize {
    //     self.index
    // }
}

impl<T> From<T> for PositionIterator<T>
where
    T: Iterator<Item = Result<char>>,
{
    fn from(it: T) -> Self {
        PositionIterator {
            it,
            index: 0,
            line: 1,
            col: 1,
        }
    }
}
