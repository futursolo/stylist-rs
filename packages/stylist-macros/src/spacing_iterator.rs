//! Provide an iterator inserting optional items between items

#[test]
fn test_spacing_iterator() {
    use SpacedIterator;
    let it = (1..7).spaced_with(|l, _| (*l == 4).then_some(2000));
    itertools::assert_equal(it, vec![1, 2, 3, 4, 2000, 5, 6]);
}

pub trait SpacedIterator: Iterator {
    /// Space a sequence of items by sometimes inserting another item.
    fn spaced_with<F>(self, spacer: F) -> Spacing<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Self::Item>,
    {
        Spacing {
            it: self,
            state: SpacingState::NotStarted,
            spacer,
        }
    }
}

impl<I: Iterator> SpacedIterator for I {}

enum SpacingState<I> {
    NotStarted,
    AtItemSpaced { item: I, spacing: I, next: I },
    AtItem { item: I, next: I },
    AtSpacing { spacing: I, next: I },
    AtEnd { item: I },
    Done,
}

impl<I> SpacingState<I> {
    fn maybe_spaced(
        item: I,
        next: Option<I>,
        spacer: &mut impl FnMut(&I, &I) -> Option<I>,
    ) -> Self {
        match next {
            None => Self::AtEnd { item },
            Some(next) => match spacer(&item, &next) {
                None => Self::AtItem { item, next },
                Some(spacing) => Self::AtItemSpaced {
                    item,
                    spacing,
                    next,
                },
            },
        }
    }

    fn advance(
        self,
        it: &mut impl Iterator<Item = I>,
        spacer: &mut impl FnMut(&I, &I) -> Option<I>,
    ) -> (Option<I>, Self) {
        use SpacingState::*;
        match self {
            NotStarted => match it.next() {
                None => (None, Done),
                Some(item) => Self::maybe_spaced(item, it.next(), spacer).advance(it, spacer),
            },
            Done => (None, Done),
            AtEnd { item } => (Some(item), Done),
            AtSpacing { spacing, next } => {
                (Some(spacing), Self::maybe_spaced(next, it.next(), spacer))
            }
            AtItem { item, next } => (Some(item), Self::maybe_spaced(next, it.next(), spacer)),
            AtItemSpaced {
                item,
                spacing,
                next,
            } => (Some(item), Self::AtSpacing { spacing, next }),
        }
    }
}

pub struct Spacing<I: Iterator, F> {
    it: I,
    state: SpacingState<I::Item>,
    spacer: F,
}

impl<It, F> Iterator for Spacing<It, F>
where
    It: Iterator,
    F: FnMut(&It::Item, &It::Item) -> Option<It::Item>,
{
    type Item = It::Item;

    fn next(&mut self) -> Option<It::Item> {
        // In case of panic in advance, drop the remaining items
        let state = std::mem::replace(&mut self.state, SpacingState::Done);
        let (next, new_state) = state.advance(&mut self.it, &mut self.spacer);
        self.state = new_state;
        next
    }
}
