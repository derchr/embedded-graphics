//! A line segment constructed from two line joints.
//!
//! Segment always has one open side, so scanline intersections may return one or two points, but
//! no more.

use super::line::Side;
use crate::{
    geometry::Point,
    primitives::{line_joint::JointKind, line_joint::LineJoint, Line},
    style::StrokeAlignment,
};

// /// TODO: Doc
// #[derive(Debug, Clone, Copy)]
// pub enum SegmentIntersection {
//     /// Start and end intersection.
//     Closed(Line),

//     /// Only one intersection, could be start or end. Usually 1px long but for shallow lines this
//     /// can be longer.
//     Open(Line),
// }

/// TODO: Doc
#[derive(Debug, Clone, Copy)]
pub struct ThickSegment {
    start_joint: LineJoint,
    end_joint: LineJoint,
}

impl ThickSegment {
    /// New
    pub fn new(start_joint: LineJoint, end_joint: LineJoint) -> Self {
        Self {
            start_joint,
            end_joint,
        }
    }

    /// Get up to 6 lines comprising the perimiter of this segment.
    fn perimiter(&self) -> [Option<Line>; 6] {
        let start_cap = self.start_joint.start_cap_lines();
        let end_cap = self.end_joint.end_cap_lines();

        [
            start_cap[0],
            start_cap[1],
            end_cap[0],
            end_cap[1],
            Line::new(
                self.start_joint.second_edge_start.right,
                self.end_joint.first_edge_end.right,
            )
            .into(),
            Line::new(
                self.end_joint.first_edge_end.left,
                self.start_joint.second_edge_start.left,
            )
            .into(),
        ]
    }

    /// TODO: DOc
    pub fn intersection(&self, scanline_y: i32) -> Option<Line> {
        let perimiter = self.perimiter();

        let it = perimiter.iter().filter_map(|l| {
            l.and_then(|l| {
                l.bresenham_scanline_intersection(scanline_y)
                    .map(|(intersection, _)| intersection.as_line().sorted_x())
            })
        });

        let line = it.fold(None, |acc: Option<Line>, line| {
            if let Some(acc) = acc {
                Some(Line::new(
                    acc.start.component_min(line.start),
                    acc.end.component_max(line.end),
                ))
            } else {
                Some(line)
            }
        })?;

        Some(line)
    }
}

// #[derive(Debug, Copy, Clone)]
// enum State {
//     StartEdge1,
//     StartEdge2,
//     RightEdge,
//     EndEdge1,
//     EndEdge2,
//     LeftEdge,
//     NextSegment,
//     Done,
// }

// /// Line joints iter.
// ///
// /// Iterates over all line segments that make up the closed boundary polygon of a thick polyline.
// /// Lines must be returned in increasing X order so that scanline intersections work correctly.
// #[derive(Clone, Copy, Debug)]
// pub struct Segments {
//     state: State,
//     start_joint: LineJoint,
//     end_joint: LineJoint,
// }

// impl Segments {
//     /// TODO: Unpub me
//     pub fn new(start_joint: LineJoint, end_joint: LineJoint) -> Self {
//         Self {
//             start_joint,
//             end_joint,
//             state: State::StartEdge,
//         }
//     }
// }

// impl Iterator for Segments {
//     type Item = Line;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.state {
//             State::StartEdge => {
//                 self.state = State::LeftEdge;

//                 Some(Line::new(
//                     self.start_joint.first_edge_end.left,
//                     self.start_joint.first_edge_end.right,
//                 ))
//             }
//             State::LeftEdge => {
//                 self.state = State::RightEdge;

//                 Some(Line::new(
//                     self.start_joint.second_edge_start.left,
//                     self.end_joint.first_edge_end.left,
//                 ))
//             }
//             State::RightEdge => {
//                 self.state = match self.end_joint.kind {
//                     JointKind::Bevel { .. } | JointKind::Degenerate { .. } => State::Bevel,
//                     _ => State::EndEdge,
//                 };

//                 Some(Line::new(
//                     self.start_joint.second_edge_start.right,
//                     self.end_joint.first_edge_end.right,
//                 ))
//             }
//             State::Bevel => {
//                 self.state = State::ClosingLine;

//                 match self.end_joint.kind {
//                     JointKind::Bevel { filler_line, .. }
//                     | JointKind::Degenerate { filler_line, .. } => Some(dbg!(filler_line)),
//                     _ => None,
//                 }
//             }
//             State::EndEdge => {
//                 self.state = State::Done;

//                 Some(Line::new(
//                     self.end_joint.second_edge_start.left,
//                     self.end_joint.second_edge_start.right,
//                 ))
//             }
//             State::ClosingLine => {
//                 self.state = State::Done;

//                 match self.end_joint.kind {
//                     JointKind::Bevel {
//                         filler_line, side, ..
//                     }
//                     | JointKind::Degenerate {
//                         filler_line, side, ..
//                     } => {
//                         let end = match side {
//                             Side::Left => self.end_joint.second_edge_start.right,
//                             Side::Right => self.end_joint.second_edge_start.left,
//                         };

//                         Some(Line::new(filler_line.end, end))
//                     }
//                     _ => None,
//                 }
//             }
//             State::Done => None,
//         }
//     }
// }