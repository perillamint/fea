
pub struct Dendrite {
    source: Neuron,
    weight: i32
}

pub struct Neuron {
    sources: Vec<Dendrite>,
    threshold: i32,
    result_new: i32,
    pub result: i32
}

impl Neuron {
    pub fn new(threshold: i32) -> Neuron {
        Neuron {
            sources: Vec::new(),
            threshold: threshold,
            result_new: 0,
            result: 0
        }
    }

    pub fn add_dendrite(&self, dendrite: Dendrite) {
        //TODO: Impl add_dendrite
    }

    pub fn remove_dendrite(&self, dendrite: Dendrite) {
        //TODO: impl.
    }

    pub fn do_calculation() {
        //TODO: Impl do_calculation
    }

    pub fn expose_result(&self) {
        self.result = self.result_new;
    }
}

#[test]
fn hello() {
    println!("Hello!");
}

