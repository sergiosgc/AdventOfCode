const std = @import("std");
const stdin = @import("stdinIter.zig");
const Vec2D = @import("vec2d.zig").Vec2D;
const Coord2D = @import("coord2d.zig").Coord2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

fn string_to_integers(s: []const u8) !struct { i64, i64 } {
    var start: struct { usize, usize } = .{ @as(usize, 0), @as(usize, 0) };
    var end: struct { usize, usize } = .{ @as(usize, 0), @as(usize, 0) };
    while (s[start[0]] < '0' or s[start[0]] > '9') start[0] += 1;
    end[0] = start[0];
    while (!(s[end[0]] < '0' or s[end[0]] > '9')) end[0] += 1;
    start[1] = end[0];
    while (s[start[1]] < '0' or s[start[1]] > '9') start[1] += 1;
    end[1] = start[1];
    while (end[1] < s.len and !(s[end[1]] < '0' or s[end[1]] > '9')) end[1] += 1;
    return .{
        try std.fmt.parseInt(i64, s[start[0]..end[0]], 10),
        try std.fmt.parseInt(i64, s[start[1]..end[1]], 10),
    };
}

fn linear_solver(eq_int: [2][3]i64) ?[2]i64 {
    var eq_f: [2][3]f64 = undefined;
    for (0..2) |y| {
        for (0..3) |x| eq_f[y][x] = @floatFromInt(eq_int[y][x]);
    }
    var coefficient = eq_f[0][0];
    for (0..3) |x| eq_f[0][x] /= coefficient;
    coefficient = eq_f[1][0];
    for (0..3) |x| eq_f[1][x] -= eq_f[0][x] * coefficient;
    coefficient = eq_f[1][1];
    for (0..3) |x| eq_f[1][x] /= coefficient;
    coefficient = eq_f[0][1];
    for (0..3) |x| eq_f[0][x] -= eq_f[1][x] * coefficient;

    // std.debug.print("{}", .{@abs(eq_f[0][2] - @round(eq_f[0][2])) + @abs(eq_f[1][2] - @round(eq_f[1][2]))});
    if (@abs(eq_f[0][2] - @round(eq_f[0][2])) + @abs(eq_f[1][2] - @round(eq_f[1][2])) > 0.001) return null;
    return .{
        @as(i64, @intFromFloat(@round(eq_f[0][2]))),
        @as(i64, @intFromFloat(@round(eq_f[1][2]))),
    };
}

const Problem = struct {
    a: Vec2D,
    b: Vec2D,
    target: Coord2D,
    fn read() !Problem {
        const a = try string_to_integers(try stdin.stdin.next() orelse unreachable);
        const b = try string_to_integers(try stdin.stdin.next() orelse unreachable);
        const target = try string_to_integers(try stdin.stdin.next() orelse unreachable);
        return Problem{
            .a = Vec2D{ .x = a[0], .y = a[1] },
            .b = Vec2D{ .x = b[0], .y = b[1] },
            .target = Coord2D{ .x = 10000000000000 + target[0], .y = 10000000000000 + target[1] },
        };
    }
    fn solve(self: *Problem) ?[2]i64 {
        return linear_solver(.{ .{ self.a.x, self.b.x, self.target.x }, .{ self.a.y, self.b.y, self.target.y } });
    }
};

pub fn readInput() !std.ArrayList(Problem) {
    var result = std.ArrayList(Problem).init(allocator);
    errdefer result.deinit();
    var readNextProblem = true;
    while (readNextProblem) {
        try result.append(try Problem.read());
        readNextProblem = false;
        if (try stdin.stdin.next()) |_| readNextProblem = true;
    }
    return result;
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer input.deinit();
    var result: i64 = 0;
    for (0..input.items.len) |i| {
        if (input.items[i].solve()) |buttons| result += 3 * buttons[0] + buttons[1];
    }
    std.debug.print("{any}\n", .{result});
    // std.debug.print("{any}\n", .{input.items[1].solve()});
}
