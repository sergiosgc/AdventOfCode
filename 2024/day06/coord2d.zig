const std = @import("std");
const Vec2D = @import("vec2d.zig").Vec2D;
pub const Coord2D = struct {
    x: i64,
    y: i64,
    pub fn add(self: Coord2D, other: Vec2D) Coord2D {
        return Coord2D{ .x = self.x + other.x, .y = self.y + other.y };
    }
};