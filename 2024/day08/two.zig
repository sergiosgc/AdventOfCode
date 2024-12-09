const std = @import("std");
const stdin = @import("stdinIter.zig");
const Vec2D = @import("vec2d.zig").Vec2D;
const Coord2D = @import("coord2d.zig").Coord2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

pub const Input = struct {
    antennae: std.AutoHashMap(u8, std.ArrayList(Coord2D)),
    max: Coord2D,
    fn deinit(self: *Input) void {
        var values = self.antennae.valueIterator();
        while (values.next()) |val| val.deinit();
        self.antennae.deinit();
    }
};
pub const AntennaPairIterator = struct {
    input: *Input,
    antennaeIterator: std.AutoHashMap(u8, std.ArrayList(Coord2D)).ValueIterator,
    currentFrequency: ?*std.ArrayList(Coord2D),
    i: usize,
    j: usize,
    fn init(input: *Input) AntennaPairIterator {
        return AntennaPairIterator{ .input = input, .antennaeIterator = input.antennae.valueIterator(), .currentFrequency = null, .i = 0, .j = 1 };
    }
    fn next(self: *AntennaPairIterator) ?struct { Coord2D, Coord2D } {
        if (self.currentFrequency) |antennae| {
            if (self.j >= antennae.items.len) {
                self.i += 1;
                self.j = self.i + 1;
                if (self.i >= antennae.items.len) self.currentFrequency = null;
                return self.next();
            }
            self.j += 1;
            return .{ antennae.items[self.i], antennae.items[self.j - 1] };
        } else {
            self.currentFrequency = self.antennaeIterator.next();
            self.i = 0;
            self.j = 1;
            if (self.currentFrequency == null) return null;
            return self.next();
        }
        unreachable;
    }
};

pub fn readInput() !Input {
    var result = Input{ .antennae = std.AutoHashMap(u8, std.ArrayList(Coord2D)).init(allocator), .max = Coord2D{ .x = 0, .y = 0 } };
    errdefer result.deinit();
    var lineNumber: u64 = 0;
    while (try stdin.stdin.next()) |line| {
        result.max.y = @max(result.max.y, @as(i64, @intCast(lineNumber)));
        for (line, 0..) |ch, columnNumber| {
            result.max.x = @max(result.max.x, @as(i64, @intCast(columnNumber)));
            if (ch == '.') continue;
            if (result.antennae.get(ch)) |list| {
                var appendedList = list;
                try appendedList.append(Coord2D{ .x = @as(i64, @intCast(columnNumber)), .y = @as(i64, @intCast(lineNumber)) });
                _ = result.antennae.remove(ch);
                try result.antennae.put(ch, appendedList);
            } else {
                var list = std.ArrayList(Coord2D).init(allocator);
                try list.append(Coord2D{ .x = @as(i64, @intCast(columnNumber)), .y = @as(i64, @intCast(lineNumber)) });
                try result.antennae.put(ch, list);
            }
        }
        lineNumber += 1;
    }
    return result;
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input: Input = try readInput();
    defer _ = input.deinit();
    var pairs = AntennaPairIterator.init(&input);
    var antinodes = std.AutoHashMap(Coord2D, void).init(allocator);
    defer _ = antinodes.deinit();
    while (pairs.next()) |pair| {
        const v = pair[0].subtract(pair[1]);
        for (0..100) |delta_usize| {
            const delta = @as(i64, @intCast(delta_usize)) - 50;
            const antinode = pair[0].add(v.multiply(delta));
            if (antinode.x >= 0 and antinode.x <= input.max.x and antinode.y >= 0 and antinode.y <= input.max.y) try antinodes.put(antinode, {});
        }
    }
    std.debug.print("{any}\n", .{antinodes.count()});
}
