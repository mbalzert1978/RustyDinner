pub trait AggregateRoot {}

pub trait Entity<TId>: PartialEq
where
    TId: PartialEq,
{
    fn id(&self) -> &TId;

    fn equals(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
