const std = @import("std");
const Vec2D = @import("vec2d.zig").Vec2D;
const Coord2D = @import("coord2d.zig").Coord2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

const Input = struct {
    heights: [][]u8,
    fn get(self: *Input, at: Coord2D) i64 {
        return @intCast(self.heights[@as(usize, @intCast(at.y))][@as(usize, @intCast(at.x))]);
    }
    fn deinit(self: *Input) void {
        for (self.heights) |row| {
            allocator.free(row);
        }
        allocator.free(self.heights);
    }
    fn trail_heads(self: *Input) !std.ArrayList(Coord2D) {
        var result = std.ArrayList(Coord2D).init(allocator);
        errdefer result.deinit();
        for (0..self.heights.len) |y|
            for (0..self.heights[y].len) |x| {
                const coord = Coord2D{ .x = @intCast(x), .y = @intCast(y) };
                if (self.get(coord) == 0) try result.append(coord);
            };
        return result;
    }
    fn deinit_hike(hike_: *std.AutoHashMap(Coord2D, std.AutoHashMap(Coord2D, u64))) void {
        var iterator = hike_.valueIterator();
        while (iterator.next()) |value| value.deinit();
        hike_.deinit();
    }
    fn visited_put(visited: *std.AutoHashMap(Coord2D, std.AutoHashMap(Coord2D, u64)), from: Coord2D, to: Coord2D, count: u64) !void {
        if (!visited.contains(from)) {
            const hm = std.AutoHashMap(Coord2D, u64).init(allocator);
            try visited.put(from, hm);
        }
        if (visited.getEntry(from)) |*inner_entry| {
            var inner = inner_entry.value_ptr;
            if (!inner.contains(to)) try inner.put(to, 0);
            if (inner.getEntry(to)) |*c| c.value_ptr.* += count;
        }
    }
    fn hike_recursive(self: *Input, visited: *std.AutoHashMap(Coord2D, std.AutoHashMap(Coord2D, u64)), pos: Coord2D) !std.AutoHashMap(Coord2D, u64) {
        if (visited.get(pos)) |*result| return result.clone();
        const height: i64 = self.get(pos);
        if (height == 9) {
            try visited_put(visited, pos, pos, 1);
        } else {
            const directions = try Vec2D.orthogonal_directions(allocator);
            defer directions.deinit();
            for (directions.items) |direction| {
                const next_pos = pos.add(direction);
                if (next_pos.x < 0 or next_pos.y < 0 or next_pos.x >= self.heights[0].len or next_pos.y >= self.heights.len or self.get(next_pos) - height != 1) continue;
                var next_map = try self.hike_recursive(visited, next_pos);
                errdefer next_map.deinit();
                defer next_map.deinit();
                var result_to_merge = next_map.iterator();
                var to_put = std.ArrayList(struct { Coord2D, u64 }).init(allocator);
                errdefer to_put.deinit();
                while (result_to_merge.next()) |entry| {
                    const dest = entry.key_ptr;
                    const path_count = entry.value_ptr;
                    try to_put.append(.{ dest.*, path_count.* });
                }
                for (to_put.items) |item| try visited_put(visited, pos, item[0], item[1]);
                to_put.deinit();
            }
        }
        if (!visited.contains(pos)) {
            const hm = std.AutoHashMap(Coord2D, u64).init(allocator);
            try visited.put(pos, hm);
        }
        if (visited.getEntry(pos)) |entry| {
            return entry.value_ptr.clone();
        } else {
            unreachable;
        }
    }
    fn hike(self: *Input, pos: Coord2D) !std.AutoHashMap(Coord2D, u64) {
        var visited = std.AutoHashMap(Coord2D, std.AutoHashMap(Coord2D, u64)).init(allocator);
        errdefer Input.deinit_hike(&visited);
        const result = try self.hike_recursive(&visited, pos);
        Input.deinit_hike(&visited);
        return result;
    }
};

pub fn readInput() !Input {
    const stdin = std.io.getStdIn().reader();
    var buffer: [1024]u8 = undefined;
    var matrix = std.ArrayList([]u8).init(allocator);
    errdefer matrix.deinit();
    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        var row = try allocator.alloc(u8, line.len);
        errdefer allocator.free(row);
        for (line, 0..) |char, i| {
            row[i] = char - '0';
        }
        try matrix.append(row);
    }

    return Input{ .heights = try matrix.toOwnedSlice() };
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer input.deinit();
    var trail_heads = try input.trail_heads();
    defer trail_heads.deinit();
    var result: i64 = 0;
    for (trail_heads.items) |trail_head| {
        var hike = try input.hike(trail_head);
        errdefer hike.deinit();
        // var hike_iterator = hike.keyIterator();
        // while (hike_iterator.next()) |top| std.debug.print("{any}\n", .{top.*});
        // std.debug.print("{any}: {}\n", .{ trail_head, hike.count() });
        result += hike.count();
        hike.deinit();
    }
    std.debug.print("{}\n", .{result});
}
