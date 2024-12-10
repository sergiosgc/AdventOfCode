const std = @import("std");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

const DefragIterator: type = struct {
    condensed_fat: []u8,
    i: usize,
    j: usize,
    ids: []i64,
    expanded_fat_index: i64 = 0,
    fn init(condensed_fat: []u8) !DefragIterator {
        const condensed_fat_clone: []u8 = try allocator.alloc(u8, condensed_fat.len);
        const ids: []i64 = try allocator.alloc(i64, condensed_fat.len);
        @memcpy(condensed_fat_clone, condensed_fat);
        @memset(ids, 0);
        var i: i64 = 0;
        for (0..1 + ids.len / 2) |index| {
            ids[index * 2] = i;
            i += 1;
        }
        return DefragIterator{ .condensed_fat = condensed_fat_clone, .i = 0, .j = @divTrunc(condensed_fat.len - 1, 2) * 2, .ids = ids };
    }
    fn deinit(self: *DefragIterator) void {
        allocator.free(self.condensed_fat);
        allocator.free(self.ids);
    }
    fn next(self: *DefragIterator) ?i64 {
        if ((self.i % 2 == 1 and self.j <= self.i) or self.i >= self.condensed_fat.len) return null;
        if (self.condensed_fat[self.i] == '0') {
            self.i += 1;
            return self.next();
        }
        if (self.i % 2 == 0) {
            if (self.condensed_fat[self.i] == '0') {
                self.i += 1;
                return self.next();
            }
            self.expanded_fat_index += 1;
            self.condensed_fat[self.i] -= 1;
            return (self.expanded_fat_index - 1) * self.ids[self.i];
        } else {
            if (self.condensed_fat[self.i] == '0') {
                self.i += 1;
                return self.next();
            }
            if (self.condensed_fat[self.j] == '0') {
                self.j -= 2;
                return self.next();
            }
            self.ids[self.i] = self.ids[self.j];
            self.expanded_fat_index += 1;
            self.condensed_fat[self.i] -= 1;
            self.condensed_fat[self.j] -= 1;
            return (self.expanded_fat_index - 1) * self.ids[self.i];
        }
    }
};

pub fn readInput() ![]u8 {
    var buffer: [256 * 1024]u8 = undefined;
    return try std.io.getStdIn().reader().readUntilDelimiter(&buffer, '\n');
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    const input = try readInput();
    var defraggedIterator = try DefragIterator.init(input);
    defer _ = defraggedIterator.deinit();
    var result: i64 = 0;
    while (defraggedIterator.next()) |checksum| result += checksum;
    std.debug.print("{}\n", .{result});
}
