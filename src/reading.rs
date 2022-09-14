use std::str::FromStr;
pub trait Reading {
    fn read_line(&mut self) -> String;

    fn read<T>(&mut self) -> T
    where
        T: FromStr + Default,
    {
        self.read_line().parse().unwrap_or(T::default())
    }

    fn read_vec<T>(&mut self, size: Option<usize>) -> Vec<T>
    where
        T: FromStr + Default,
    {
        let txt = self.read_line();
        let wrds = txt.split_whitespace();
        let mut vc = Vec::with_capacity(size.unwrap_or(0));
        for wrd in wrds {
            let val = wrd.parse().unwrap_or(T::default());
            vc.push(val);
        }
        vc
    }
}
