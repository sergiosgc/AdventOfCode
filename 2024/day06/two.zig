const std = @import("std");
const stdin = @import("stdinIter.zig");
const Coord2D = @import("coord2d.zig").Coord2D;
const Vec2D = @import("vec2d.zig").Vec2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

pub const Visit = struct { pos: Coord2D, dir: Vec2D };

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

pub fn causesALoop(obstacles: *std.AutoHashMap(Coord2D, void), obstacle: Coord2D, visited: *std.AutoHashMap(Visit, void), maxPos: Coord2D, currentPos: Coord2D, currentDir: Vec2D) bool {
    if (currentPos.x > maxPos.x or currentPos.y > maxPos.y or currentPos.x < 0 or currentPos.y < 0) return false;
    if (visited.contains(Visit{ .pos = currentPos, .dir = currentDir })) return true;
    const nextPos = currentPos.add(currentDir);
    if (obstacles.contains(nextPos) or (nextPos.x == obstacle.x and nextPos.y == obstacle.y)) return causesALoop(obstacles, obstacle, visited, maxPos, currentPos, currentDir.rotateRight());
    visited.put(Visit{ .pos = currentPos, .dir = currentDir }, {}) catch {};
    return causesALoop(obstacles, obstacle, visited, maxPos, nextPos, currentDir);
}
pub fn causesALoopOuter(obstacles: *std.AutoHashMap(Coord2D, void), obstacle: Coord2D, maxPos: Coord2D, currentPos: Coord2D, currentDir: Vec2D) bool {
    var visited = std.AutoHashMap(Visit, void).init(allocator);
    defer _ = visited.deinit();
    return causesALoop(obstacles, obstacle, &visited, maxPos, currentPos, currentDir);
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer input.obstacles.deinit();
    var result: i64 = 0;
    for (0..@intCast(input.lowerRight.x + 1)) |x| {
        for (0..@intCast(input.lowerRight.y + 1)) |y| {
            if (causesALoopOuter(&input.obstacles, Coord2D{ .x = @intCast(x), .y = @intCast(y) }, input.lowerRight, input.startPosition, Vec2D{ .x = 0, .y = -1 })) result += 1;
        }
    }
    std.debug.print("{}", .{result});
}
