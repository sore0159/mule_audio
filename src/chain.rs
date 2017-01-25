use wave::Wave;

pub struct Chain(Vec<Wave>);

impl Iterator for Chain {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        loop {
            {
                let mut i_maybe = self.0.last_mut();
                if let Some(ref mut i) = i_maybe {
                    let val = i.next();
                    if val.is_some() {
                        return val;
                    }
                } else {
                    return None;
                }
            }
            self.0.pop();
        }
    }
}

impl Chain {
    pub fn new() -> Self {
        Chain(Vec::new())
    }
    pub fn from_iter<T>(wvs: T) -> Self
        where T: IntoIterator<Item = Wave>
    {
        let mut s = Chain(wvs.into_iter().collect());
        s.0.reverse();
        s
    }
    pub fn chain(mut self, wv: Wave) -> Self {
        self.0.insert(0, wv);
        self
    }
}
