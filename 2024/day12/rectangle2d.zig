const std = @import("std");
const Coord2D = @import("coord2d.zig").Coord2D;
pub const Rectangle2D = struct {
    min: Coord2D,
    max: Coord2D,
    pub fn init(a: Coord2D, b: Coord2D) Rectangle2D {
        return Rectangle2D{
            .min = Coord2D{ .x = @min(a.x, b.x), .y = @min(a.y, b.y) },
            .max = Coord2D{ .x = @max(a.x, b.x), .y = @max(a.y, b.y) },
        };
    }
    pub fn iterator(self: Rectangle2D) Iterator {
        return Iterator.init(self);
    }
};
pub const Iterator = struct {
    rectangle: Rectangle2D,
    current: Coord2D,
    pub fn init(rectangle: Rectangle2D) Iterator {
        return Iterator{ .rectangle = rectangle, .current = rectangle.min };
    }
    pub fn next(self: *Iterator) ?Coord2D {
        if (self.current.y > self.rectangle.max.y) return null;
        const result = self.current;
        self.current.x += 1;
        if (self.current.x > self.rectangle.max.x) {
            self.current.y += 1;
            self.current.x = self.rectangle.min.x;
        }
        return result;
    }
};
