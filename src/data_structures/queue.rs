struct Queue {
    elements: Vec<u8>
}

impl Queue {

    pub fn new() -> Self {
        Queue {
            elements: vec![]
        }
    }

    pub fn poll(&self) -> u8 {
        self.elements.first()
    }

    pub fn push(&mut self, el: u8) {
        self.elements.append(el);
    }

}
