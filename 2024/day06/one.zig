const std = @import("std");
const stdin = @import("stdinIter.zig");
const Coord2D = @import("coord2d.zig").Coord2D;
const Vec2D = @import("vec2d.zig").Vec2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

pub fn readInput() !struct { obstacles: std.AutoHashMap(Coord2D, void), startPosition: Coord2D, lowerRight: Coord2D } {
    var obstacles = std.AutoHashMap(Coord2D, void).init(allocator);
    errdefer obstacles.deinit();
    var startPosition: Coord2D = undefined;
    var lowerRight = Coord2D{ .x = 0, .y = 0 };
    var lineNumber: u64 = 0;
    while (try stdin.stdin.next()) |line| {
        lowerRight.y = @max(lowerRight.y, @as(i64, @intCast(lineNumber)));
        for (line, 0..) |ch, columnNumber| {
            lowerRight.x = @max(lowerRight.x, @as(i64, @intCast(columnNumber)));
            if (ch == '#') try obstacles.put(Coord2D{ .x = @intCast(columnNumber), .y = @intCast(lineNumber) }, {});
            if (ch == '^') startPosition = Coord2D{ .x = @intCast(columnNumber), .y = @intCast(lineNumber) };
        }
        lineNumber += 1;
    }
    return .{ .obstacles = obstacles, .startPosition = startPosition, .lowerRight = lowerRight };
}

pub fn visitedToOutofBounds(obstacles: *std.AutoHashMap(Coord2D, void), visited: *std.AutoHashMap(Coord2D, void), maxPos: Coord2D, currentPos: Coord2D, currentDir: Vec2D) i64 {
    if (currentPos.x > maxPos.x or currentPos.y > maxPos.y or currentPos.x < 0 or currentPos.y < 0) return visited.count();
    visited.put(currentPos, {}) catch {};
    const nextPos = currentPos.add(currentDir);
    if (obstacles.contains(nextPos)) return visitedToOutofBounds(obstacles, visited, maxPos, currentPos, currentDir.rotateRight());
    return visitedToOutofBounds(obstacles, visited, maxPos, nextPos, currentDir);
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer input.obstacles.deinit();
    var visited = std.AutoHashMap(Coord2D, void).init(allocator);
    defer _ = visited.deinit();
    std.debug.print("{}", .{visitedToOutofBounds(&input.obstacles, &visited, input.lowerRight, input.startPosition, Vec2D{ .x = 0, .y = -1 })});
}
