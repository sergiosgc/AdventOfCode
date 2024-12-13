const std = @import("std");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

pub fn hm_incr(hm: *std.AutoHashMap(u64, usize), key: u64, howMuch: usize) !void {
    if (hm.getEntry(key)) |entry| {
        entry.value_ptr.* += howMuch;
    } else {
        try hm.put(key, howMuch);
    }
}

pub fn readInput() !std.AutoHashMap(u64, usize) {
    var result = std.AutoHashMap(u64, usize).init(allocator);
    var buffer: [256 * 1024]u8 = undefined;
    const line = try std.io.getStdIn().reader().readUntilDelimiter(&buffer, '\n');
    var it = std.mem.splitAny(u8, line, " ");
    while (it.next()) |strval| try hm_incr(&result, try std.fmt.parseInt(u64, strval, 10), 1);
    return result;
}

pub fn blink(from: *std.AutoHashMap(u64, usize)) !std.AutoHashMap(u64, usize) {
    var result = std.AutoHashMap(u64, usize).init(allocator);
    errdefer result.deinit();
    var iterator = from.iterator();
    var value_as_string_buffer: [4096]u8 = undefined;
    while (iterator.next()) |entry| {
        const value_as_string = try std.fmt.bufPrint(&value_as_string_buffer, "{}", .{entry.key_ptr.*});
        if (entry.key_ptr.* == 0) {
            try hm_incr(&result, 1, entry.value_ptr.*);
        } else if (value_as_string.len % 2 == 0) {
            const left = value_as_string[0 .. value_as_string.len / 2];
            const right = value_as_string[value_as_string.len / 2 ..];
            try hm_incr(&result, try std.fmt.parseInt(u64, left, 10), entry.value_ptr.*);
            try hm_incr(&result, try std.fmt.parseInt(u64, right, 10), entry.value_ptr.*);
        } else {
            try hm_incr(&result, 2024 * entry.key_ptr.*, entry.value_ptr.*);
        }
    }
    from.*.deinit();
    return result;
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input = try readInput();
    defer _ = input.deinit();
    const blink_count: usize = 25;
    for (0..blink_count) |_| input = try blink(&input);

    var input_iterator = input.valueIterator();
    var result: usize = 0;
    while (input_iterator.next()) |count| result += count.*;
    std.debug.print("{}\n", .{result});
}
