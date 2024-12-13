const std = @import("std");

pub const Vec2D = struct {
    x: i64,
    y: i64,
    pub fn rotateRight(self: Vec2D) Vec2D {
        return Vec2D{ .x = -self.y, .y = self.x };
    }
    pub fn rotateLeft(self: Vec2D) Vec2D {
        return Vec2D{ .x = self.y, .y = -self.x };
    }
    pub fn invert(self: Vec2D) Vec2D {
        return self.multiply(-1);
    }
    pub fn multiply(self: Vec2D, other: i64) Vec2D {
        return Vec2D{ .x = other * self.x, .y = other * self.y };
    }
    pub fn orthogonal_directions(allocator: std.mem.Allocator) !std.ArrayList(Vec2D) {
        var result = std.ArrayList(Vec2D).init(allocator);
        try result.append(Vec2D{ .x = 1, .y = 0 });
        try result.append(Vec2D{ .x = -1, .y = 0 });
        try result.append(Vec2D{ .x = 0, .y = 1 });
        try result.append(Vec2D{ .x = 0, .y = -1 });
        return result;
    }
};
