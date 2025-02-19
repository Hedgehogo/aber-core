pub trait Error {
    fn merge(self, other: Self) -> Self;
}