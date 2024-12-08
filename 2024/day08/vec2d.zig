pub const Vec2D = struct {
    x: i64,
    y: i64,
    pub fn rotateRight(self: Vec2D) Vec2D {
        return Vec2D{ .x = -self.y, .y = self.x };
    }
    pub fn invert(self: Vec2D) Vec2D {
        return self.multiply(-1);
    }
    pub fn multiply(self: Vec2D, other: i64) Vec2D {
        return Vec2D{ .x = other * self.x, .y = other * self.y };
    }
};
