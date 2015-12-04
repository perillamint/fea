
pub struct Dendrite {
    source: Neuron,
    weight: i32
}

pub struct Neuron {
    dendrites: Vec<Dendrite>,
    threshold: i32,
    result_new: i32,
    pub result: i32
}

impl PartialEq for Neuron {
    fn eq(&self, other: &Neuron) -> bool {
        self == other
    }

    fn ne(&self, other: &Neuron) -> bool {
        !self.eq(other)
    }
}

impl Neuron {
    pub fn new(threshold: i32) -> Neuron {
        Neuron {
            dendrites: Vec::new(),
            threshold: threshold,
            result_new: 0,
            result: 0
        }
    }

    fn search_dendrite(&self, neuron: &Neuron) -> Option<usize> {
        let mut iter = self.dendrites.iter();
        let mut cnt: usize = 0;

        loop {
            match iter.next() {
                Some(dendrite) => {
                    if dendrite.source == *neuron {
                        return Some(cnt);
                    }
                },
                None => return None,
            }

            cnt += 1;
        }
    }

    pub fn add_dendrite(&mut self, dendrite: Dendrite) {
        let idx = self.search_dendrite(&dendrite.source);

        match idx {
            Some(_) => panic!("Duplicated dendrites!"),
            None => self.dendrites.push(dendrite),
        }
    }

    pub fn remove_dendrite(&mut self, neuron: Neuron) {
        let idx = self.search_dendrite(&neuron);

        match idx {
            Some(x) => {
                self.dendrites.remove(x);
            },
            None => return,
        }
    }

    pub fn do_calculation(&mut self) {
        let mut iter = self.dendrites.iter();
        let mut acc: i32 = 0;

        loop {
            match iter.next() {
                Some(dendrite) => {
                    acc += dendrite.source.result * dendrite.weight;
                },
                None => break,
            }
        }

        if acc > self.threshold {
            self.result_new = 1;
        } else {
            self.result_new = 0;
        }
    }

    pub fn expose_result(&mut self) {
        self.result = self.result_new;
    }
}

#[test]
fn hello() {
    println!("Hello!");
}

