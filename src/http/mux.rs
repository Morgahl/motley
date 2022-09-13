// use std::fmt::Debug;

// use anyhow::Result;
// use http::{Request, Response};

// fn serve(request: Request<()>) -> Result<Response<()>> {
//     let path_iter = request.uri().path().split('/');

//     // match ()
//     Ok(Response::new(()))
// }

// enum TripleIteratorState<'a, I>
// where
//     I: Iterator,
// {
//     Init,
//     Empty,
//     Single(Option<&'a I::Item>),
//     Double(Option<&'a I::Item>, Option<&'a I::Item>),
//     Triple(
//         Option<&'a I::Item>,
//         Option<&'a I::Item>,
//         Option<&'a I::Item>,
//     ),
// }

// impl<'a, I> TripleIteratorState<'a, I>
// where
//     I: Iterator,
// {
//     fn new(iter: &mut I) -> TripleIteratorState<I>
//     where
//         I: Iterator,
//         I::Item: Clone + Debug,
//     {
//         let one = iter.next();
//         let two = iter.next();
//         let three = iter.next();
//         match (one, two, three) {
//             (Some(_), Some(_), Some(_)) => {
//                 TripleIteratorState::Triple(one.as_ref(), two.as_ref(), three.as_ref())
//             }
//             (Some(_), Some(_), None) => TripleIteratorState::Double(one.as_ref(), two.as_ref()),
//             (Some(_), None, None) => TripleIteratorState::Single(one.as_ref()),
//             (None, None, None) => TripleIteratorState::Empty,
//             _ => TripleIteratorState::Empty,
//         }
//     }
// }

// ! IMPLEMENTATION ASSUMES PASSED ITERATOR ONLY EMITS `NONE` AFTER THE FIRST INSTANCE OF A `NONE`
// struct TripleIterator<I>(I, TripleIteratorState<I>)
// where
//     I: Iterator,
//     I::Item: Clone + Debug;

// impl<I> TripleIterator<I>
// where
//     I: Iterator,
//     I::Item: Clone + Debug,
// {
//     pub fn new(iter: I) -> Self {
//         Self(iter, TripleIteratorState::Init)
//     }

//     fn next(&mut self) -> Option<(Option<&I::Item>, Option<&I::Item>, Option<&I::Item>)> {
//         match self.1 {
//             TripleIteratorState::Init => {
//                 self.1 =
//                     TripleIteratorState::new(&mut self.0).unwrap_or(TripleIteratorState::Empty);
//                 self.next()
//             }
//             TripleIteratorState::Empty => None,
//             TripleIteratorState::Single(first) => {
//                 self.1 = TripleIteratorState::Empty;
//                 Some((first.as_ref(), None, None))
//             }
//             TripleIteratorState::Double(first, second) => {
//                 self.1 = TripleIteratorState::Single(second);
//                 Some((first.as_ref(), second.as_ref(), None))
//             }
//             TripleIteratorState::Triple(first, second, third) => {
//                 match self.0.next() {
//                     Some(next) => self.1 = TripleIteratorState::Triple(second, third, Some(&next)),
//                     None => self.1 = TripleIteratorState::Double(second, third),
//                 }
//                 Some((first.as_ref(), second.as_ref(), third.as_ref()))
//             }
//         }
//     }
// }
