//! Math utilities on `Position` which don't exist in the Screeps API
//! proper.
use std::ops::{Add, Sub};

use super::Position;
use crate::{constants::Direction, local::position::WorldPositionOutOfBoundsError};

impl Position {
    /// Returns whether this coordinate represents a room edge position (0 or
    /// 49)
    pub fn is_room_edge(self) -> bool {
        self.xy().is_room_edge()
    }

    /// Gets Manhattan range to the specified position.
    ///
    /// While most calculations in Screeps: World use Chebyshev distance (see
    /// [`Position::get_range_to`]), the Manhattan distance between two
    /// positions can be useful for certain calculations.
    ///
    /// For more information see [Wikipedia](https://en.wikipedia.org/wiki/Taxicab_geometry).
    ///
    /// Like other [`Position`] math functions, this operates on positions as
    /// "world positions", and will return an accurate range for positions in
    /// different rooms.
    ///
    /// # Examples
    /// ```rust
    /// # use screeps::{Position, RoomCoordinate};
    /// let w127n127 = "W127N127".parse().unwrap();
    /// let w126n126 = "W126N126".parse().unwrap();
    /// let w127s127 = "W127S127".parse().unwrap();
    /// let e127s127 = "E127S127".parse().unwrap();
    /// 
    /// let pos_top_left = Position::new(
    ///    RoomCoordinate::new(0).unwrap(),
    ///    RoomCoordinate::new(0).unwrap(),
    ///    w127n127,
    /// );
    /// let pos_diagonal_room = Position::new(
    ///    RoomCoordinate::new(0).unwrap(),
    ///    RoomCoordinate::new(0).unwrap(),
    ///    w126n126,
    /// );
    /// let pos_bot_left_zero = Position::new(
    ///    RoomCoordinate::new(0).unwrap(),
    ///    RoomCoordinate::new(0).unwrap(),
    ///    w127s127,
    /// );
    /// let pos_bot_right_max = Position::new(
    ///    RoomCoordinate::new(49).unwrap(),
    ///    RoomCoordinate::new(49).unwrap(),
    ///    e127s127,
    /// );
    /// 
    /// assert_eq!(pos_top_left.get_manhattan_range_to(pos_diagonal_room), 100);
    /// // 255 rooms distance from top room to bottom room
    /// assert_eq!(pos_top_left.get_manhattan_range_to(pos_bot_left_zero), 255 * 50);
    /// // and that should be the range for the room one away diagonally from the top left, too
    /// assert_eq!(pos_diagonal_room.get_manhattan_range_to(pos_bot_left_zero), 255 * 50);
    /// // 255 rooms distance from top to bottom, 255 more from left to right, and 49 * 2 to the room corner
    /// assert_eq!(pos_top_left.get_manhattan_range_to(pos_bot_right_max), 255 * 50 * 2 + 49 * 2);
    /// ```
    pub fn get_manhattan_range_to(self, target: Position) -> u32 {
        let (dx, dy) = self - target;
        dx.unsigned_abs() + dy.unsigned_abs()
    }

    /// Returns a new position offset from this position by the specified x
    /// coords and y coords.
    ///
    /// This function operates on world coordinates, and will wrap between rooms
    /// if necessary.
    ///
    /// To return a new position rather than modifying in place, use `pos + (x,
    /// y)`. See the implementation of `Add<(i32, i32)>` for
    /// [`Position`] further down on this page.
    ///
    /// # Panics
    ///
    /// Will panic if the new position overflows the world. See
    /// [`Position::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// # use screeps::{Position, RoomCoordinate};
    /// let e21s21 = "E21S21".parse().unwrap();
    /// let e21s22 = "E21S22".parse().unwrap();
    ///
    /// let mut pos = Position::new(
    ///     RoomCoordinate::try_from(21).unwrap(),
    ///     RoomCoordinate::try_from(21).unwrap(),
    ///     e21s21,
    /// );
    /// pos.offset(5, 5);
    /// assert_eq!(
    ///     pos,
    ///     Position::new(
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         e21s21
    ///     )
    /// );
    ///
    /// pos.offset(0, 49);
    /// assert_eq!(
    ///     pos,
    ///     Position::new(
    ///         RoomCoordinate::try_from(26).unwrap(),
    ///         RoomCoordinate::try_from(25).unwrap(),
    ///         e21s22
    ///     )
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn offset(&mut self, x: i32, y: i32) {
        *self = *self + (x, y);
    }

    /// Adds an `(x, y)` pair to this room position's world coordinates and
    /// returns the result.
    ///
    /// Will change rooms if necessary.
    ///
    /// # Errors
    /// Returns `Err` if the new position's room is outside bounds.
    ///
    /// For a panicking variant of this function, see [`Position::add`].
    ///
    /// See [`Position::from_world_coords`].
    #[inline]
    pub fn checked_add(self, rhs: (i32, i32)) -> Result<Position, WorldPositionOutOfBoundsError> {
        let (x1, y1) = self.world_coords();
        let (x2, y2) = rhs;

        Position::checked_from_world_coords(x1 + x2, y1 + y2)
    }

    /// Adds a [`Direction`] to this room position's world coordinates and
    /// returns the result.
    ///
    /// Will change rooms if necessary.
    ///
    /// # Errors
    /// Returns `Err` if the new position's room is outside bounds.
    ///
    /// See [`Position::from_world_coords`].
    #[inline]
    pub fn checked_add_direction(
        self,
        direction: Direction,
    ) -> Result<Position, WorldPositionOutOfBoundsError> {
        let (x1, y1) = self.world_coords();
        let (x2, y2) = direction.into();

        Position::checked_from_world_coords(x1 + x2, y1 + y2)
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    /// Adds an `(x, y)` pair to this room position's world coordinates.
    ///
    /// Will change rooms if necessary.
    ///
    /// # Panics
    ///
    /// Will panic if the new position's room is outside bounds. See
    /// [`Position::from_world_coords`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use screeps::{Position, RoomCoordinate};
    /// let w5s6 = "W5S6".parse().unwrap();
    /// let w5s5 = "W5S5".parse().unwrap();
    ///
    /// let pos1 = Position::new(
    ///     RoomCoordinate::try_from(42).unwrap(),
    ///     RoomCoordinate::try_from(42).unwrap(),
    ///     w5s6,
    /// );
    /// let pos2 = pos1 + (7, 7);
    /// assert_eq!(
    ///     pos2,
    ///     Position::new(
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         w5s6
    ///     )
    /// );
    ///
    /// let pos3 = pos2 + (0, -59);
    /// assert_eq!(
    ///     pos3,
    ///     Position::new(
    ///         RoomCoordinate::try_from(49).unwrap(),
    ///         RoomCoordinate::try_from(40).unwrap(),
    ///         w5s5
    ///     )
    /// );
    ///
    /// let pos4 = pos3 - (49, 0);
    /// assert_eq!(
    ///     pos4,
    ///     Position::new(
    ///         RoomCoordinate::try_from(0).unwrap(),
    ///         RoomCoordinate::try_from(40).unwrap(),
    ///         w5s5
    ///     )
    /// );
    /// ```
    #[inline]
    #[track_caller]
    fn add(self, (x, y): (i32, i32)) -> Self {
        self.checked_add((x, y)).unwrap()
    }
}

impl Add<Direction> for Position {
    type Output = Position;
    #[inline]
    #[track_caller]
    fn add(self, direction: Direction) -> Self {
        self.checked_add_direction(direction).unwrap()
    }
}

impl Sub<(i32, i32)> for Position {
    type Output = Position;

    /// See the implementation of `Add<(i32, i32)>` for [`Position`].
    #[inline]
    #[track_caller]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        self.checked_add((-x, -y)).unwrap()
    }
}

impl Sub<Direction> for Position {
    type Output = Position;
    #[inline]
    fn sub(self, direction: Direction) -> Self {
        self.checked_add_direction(-direction).unwrap()
    }
}

impl Sub<Position> for Position {
    type Output = (i32, i32);

    /// Subtracts the other room position from this one, extracting the
    /// difference as the output.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use screeps::{Position, RoomCoordinate};
    /// let e5n5 = "E5N5".parse().unwrap();
    /// let e5n6 = "E5N6".parse().unwrap();
    ///
    /// let pos1 = Position::new(
    ///     RoomCoordinate::try_from(40).unwrap(),
    ///     RoomCoordinate::try_from(40).unwrap(),
    ///     e5n5,
    /// );
    /// let pos2 = Position::new(
    ///     RoomCoordinate::try_from(0).unwrap(),
    ///     RoomCoordinate::try_from(20).unwrap(),
    ///     e5n6,
    /// );
    /// assert_eq!(pos1 - pos2, (40, 70));
    /// ```
    #[inline]
    fn sub(self, other: Position) -> (i32, i32) {
        let (mx, my) = self.world_coords();
        let (ox, oy) = other.world_coords();
        (mx - ox, my - oy)
    }
}
