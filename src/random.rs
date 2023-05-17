use quad_rand as qrand;

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self) {
        let len = self.len();
        if len < 2 {
            return;
        }
        for i in 0..len - 1 {
            let j = qrand::gen_range(i, len);
            self.swap(i, j);
        }
    }
}
