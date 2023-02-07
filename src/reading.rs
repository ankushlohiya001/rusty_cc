use std::str::FromStr;
pub trait Reading {
    fn read_line(&mut self) -> String;

    fn read<T>(&mut self) -> T
    where
        T: FromStr + Default,
    {
        self.read_line().parse().unwrap_or(T::default())
    }

    fn read_vec<T>(&mut self) -> Vec<T>
    where
        T: FromStr + Default,
    {
        let txt = self.read_line();
        let wrds = txt.split_whitespace();
        wrds
            .map(|wrd| wrd.parse().unwrap_or(T::default()))
            .collect()
    }
}
