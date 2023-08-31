use super::Ray;

impl Ray {
    pub fn normalize(&mut self) {
        self.direction.normalize()
    }

    pub fn normalized(self) -> Self {
        Ray {
            direction: self.direction.normalized(),
            ..self
        }
    }
}
