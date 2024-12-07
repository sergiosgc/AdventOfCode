pub const Vec2D = struct {
    x: i64,
    y: i64,
    pub fn rotateRight(self: Vec2D) Vec2D {
        return Vec2D{ .x = -self.y, .y = self.x };
    }
};
