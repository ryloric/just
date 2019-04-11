use common::*;

pub trait RangeExt<T> {
  fn range_contains(&self, i: &T) -> bool;
}

impl<T> RangeExt<T> for Range<T>
where
  T: PartialOrd + Copy,
{
  fn range_contains(&self, i: &T) -> bool {
    i >= &self.start && i < &self.end
  }
}

impl<T> RangeExt<T> for RangeInclusive<T>
where
  T: PartialOrd + Copy,
{
  fn range_contains(&self, i: &T) -> bool {
    i >= self.start() && i <= self.end()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn range() {
    assert!((0..1).range_contains(&0));
    assert!((10..20).range_contains(&15));
    assert!(!(0..0).range_contains(&0));
    assert!(!(1..10).range_contains(&0));
    assert!(!(1..10).range_contains(&10));
  }

  #[test]
  fn range_inclusive() {
    assert!((0..=10).range_contains(&0));
    assert!((0..=10).range_contains(&7));
    assert!((0..=10).range_contains(&10));
    assert!(!(0..=10).range_contains(&11));
    assert!(!(5..=10).range_contains(&4));
  }
}
