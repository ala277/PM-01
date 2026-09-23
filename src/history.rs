use crate::transition::Transition;

#[derive(Debug, Default, Clone)]
pub struct History {
    pub transitions: Vec<Transition>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }

    pub fn len(&self) -> usize {
        self.transitions.len()
    }
}
