const std = @import("std");
const Vec2D = @import("vec2d.zig").Vec2D;
const Coord2D = @import("coord2d.zig").Coord2D;
const Rectangle2D = @import("rectangle2d.zig").Rectangle2D;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

pub fn hm_incr(comptime K: type, comptime V: type, hm: *std.AutoHashMap(K, V), key: K, howMuch: V) !void {
    if (hm.getEntry(key)) |entry| {
        entry.value_ptr.* += howMuch;
    } else {
        try hm.put(key, howMuch);
    }
}

const Input = struct {
    plants: [][]u8,
    area_id_seq: u64 = 0,
    area_id: std.AutoHashMap(Coord2D, u64),
    perimeter: std.AutoHashMap(u64, u64),
    area: std.AutoHashMap(u64, u64),
    fn deinit(self: *Input) void {
        for (self.plants) |row| {
            allocator.free(row);
        }
        allocator.free(self.plants);
        self.area_id.deinit();
        self.perimeter.deinit();
        self.area.deinit();
    }
    fn plant(self: *const Input, coord: Coord2D) ?u8 {
        if (coord.x < 0 or coord.y < 0 or coord.y >= self.plants.len or coord.x >= self.plants[0].len) return null;
        return self.plants[@as(usize, @intCast(coord.y))][@as(usize, @intCast(coord.x))];
    }
    fn flood_fill(self: *Input, area_id: u64, pos: Coord2D) !void {
        const current_plant = self.plant(pos) orelse unreachable;
        try self.area_id.put(pos, area_id);
        var directions = try Vec2D.orthogonal_directions(allocator);
        defer directions.deinit();
        for (directions.items) |direction| {
            const next_coord = pos.add(direction);
            if (self.plant(next_coord)) |next_plant| {
                if (!self.area_id.contains(next_coord) and next_plant == current_plant) try self.flood_fill(area_id, next_coord);
            }
        }
    }
    fn paint(self: *Input) !void {
        var coord_iterator = Rectangle2D.init(Coord2D{ .x = 0, .y = 0 }, Coord2D{ .x = @intCast(self.plants[0].len - 1), .y = @intCast(self.plants.len - 1) }).iterator();
        while (coord_iterator.next()) |current| {
            const current_plant = self.plant(current) orelse unreachable;
            const current_area_id = if (self.area_id.get(current)) |area_id| blk: {
                break :blk area_id;
            } else blk: {
                self.area_id_seq += 1;
                try self.flood_fill(self.area_id_seq, current);
                break :blk self.area_id.get(current) orelse unreachable;
            };
            try hm_incr(u64, u64, &self.area, current_area_id, 1);

            var directions = try Vec2D.orthogonal_directions(allocator);
            errdefer directions.deinit();
            for (directions.items) |direction| {
                const next_coord = current.add(direction);
                if (self.plant(next_coord)) |next_plant| {
                    if (next_plant != current_plant) try hm_incr(u64, u64, &self.perimeter, current_area_id, 1);
                } else try hm_incr(u64, u64, &self.perimeter, current_area_id, 1);
            }

            directions.deinit();
        }
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
            row[i] = char;
        }
        try matrix.append(row);
    }

    return Input{ .plants = try matrix.toOwnedSlice(), .area_id = std.AutoHashMap(Coord2D, u64).init(allocator), .perimeter = std.AutoHashMap(u64, u64).init(allocator), .area = std.AutoHashMap(u64, u64).init(allocator) };
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer input.deinit();
    try input.paint();
    // for (1..input.area_id_seq + 1) |area_id| {
    //     std.debug.print("ID: {}, ", .{area_id});
    //     std.debug.print("Area: {}, Perimeter: {}\n", .{ input.area.get(area_id) orelse unreachable, input.perimeter.get(area_id) orelse unreachable });
    // }
    var result: u64 = 0;
    for (1..input.area_id_seq + 1) |area_id| result += (input.area.get(area_id) orelse unreachable) * (input.perimeter.get(area_id) orelse unreachable);
    std.debug.print("{}\n", .{result});
}
