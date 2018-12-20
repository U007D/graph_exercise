//#[cfg(test)]
//mod unit_tests;
//use crate::event::Event;
//
//#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
//pub enum Severity {
//    NoFault,
//    Warn,
//    Severe,
//    Critical,
//}
//
//pub struct SubTreeScanner<'a, T: Event> {
//    i_edges: &'a [T],
//}
//
//impl<'a, T: Event> SubTreeScanner<T> {
//    pub fn new(i_edges: &'a [T]) -> Self {
//        Self {
//            i_edges,
//        }
//    }
//
//    fn fault_condition() -> Severity {
//        incoming_edges.iter()
//                      .fold(Severity::NoFault, |max, curr| match curr > max {
//                          true => curr,
//                          false => max,
//                      })
//    }
//}
//

