const std = @import("std");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

const DefragIterator: type = struct {
    condensed_fat: []u8,
    i: usize,
    ids: []i64,
    expanded_fat_index: i64 = 0,
    fn shiftLeft(condensed_fat: *[]u8, ids: *[]i64, left: usize, right: usize) void {
        const howMany = condensed_fat.*[right];
        const totalFree = howMany + condensed_fat.*[right - 1] + condensed_fat.*[right + 1];
        const id = ids.*[right];
        if (left + 2 <= right) {
            condensed_fat.*[left] -= howMany;
            condensed_fat.*[right + 1] = 0;
            std.mem.copyBackwards(u8, condensed_fat.*[left + 2 .. right + 1], condensed_fat.*[left .. right - 1]);
            condensed_fat.*[right + 1] = totalFree;
        } else {
            condensed_fat.*[right + 1] = howMany + condensed_fat.*[right + 1];
            condensed_fat.*[right] = 0;
        }
        condensed_fat.*[left] = 0;
        condensed_fat.*[left + 1] = howMany;
        ids.*[right + 1] = 0;
        ids.*[right] = 0;
        ids.*[right - 1] = 0;
        if (left + 2 <= right) std.mem.copyBackwards(i64, ids.*[left + 2 .. right + 1], ids.*[left .. right - 1]);
        ids.*[left] = 0;
        ids.*[left + 1] = id;
    }
    fn debug_print(fat: []u8, ids: []i64) void {
        for (0..fat.len) |i| {
            if (fat[i] > 0) for (0..fat[i]) |_| if (i % 2 == 0) {
                std.debug.print("{c}", .{'0' + @as(u8, @intCast(ids[i]))});
            } else {
                std.debug.print(".", .{});
            };
        }
        std.debug.print("\n", .{});
    }
    fn init(condensed_fat: []u8) !DefragIterator {
        var condensed_fat_clone: []u8 = try allocator.alloc(u8, condensed_fat.len + 1);
        var ids: []i64 = try allocator.alloc(i64, condensed_fat.len + 1);
        @memset(condensed_fat_clone, '0');
        @memcpy(condensed_fat_clone[0..condensed_fat.len], condensed_fat);
        for (0..condensed_fat_clone.len) |i| condensed_fat_clone[i] -= '0';
        @memset(ids, 0);
        var i: i64 = 0;
        for (0..1 + (ids.len - 1) / 2) |index| {
            ids[index * 2] = i;
            i += 1;
        }
        var right: usize = (condensed_fat_clone.len - 1) / 2 * 2;
        while (right > 0) {
            if (condensed_fat_clone[right] == 0) {
                right -= 2;
                continue;
            }
            var left: usize = 1;
            while (left < right and condensed_fat_clone[left] < condensed_fat_clone[right]) left += 2;
            if (left < right) {
                // std.debug.print("---\n{} -> {}\n", .{ right, left });
                // DefragIterator.debug_print(condensed_fat_clone, ids);
                // std.debug.print("before: {any}\n{any}\n", .{ condensed_fat_clone, ids });
                DefragIterator.shiftLeft(&condensed_fat_clone, &ids, left, right);
                // DefragIterator.debug_print(condensed_fat_clone, ids);
                // std.debug.print("after: {any}\n{any}\n", .{ condensed_fat_clone, ids });
            }
            if (right > 1) right -= 2;
        }
        // DefragIterator.debug_print(condensed_fat_clone, ids);
        // DefragIterator.shiftLeft(&condensed_fat_clone, &ids, 1, 18);
        return DefragIterator{ .condensed_fat = condensed_fat_clone, .i = 0, .ids = ids };
    }
    fn deinit(self: *DefragIterator) void {
        allocator.free(self.condensed_fat);
        allocator.free(self.ids);
    }
    fn next(self: *DefragIterator) ?i64 {
        if (self.i >= self.condensed_fat.len) return null;
        if (self.condensed_fat[self.i] == 0) {
            self.i += 1;
            return self.next();
        }
        self.condensed_fat[self.i] -= 1;
        self.expanded_fat_index += 1;
        // std.debug.print("{} * {}\n", .{ self.expanded_fat_index - 1, self.ids[self.i] });
        return (self.expanded_fat_index - 1) * self.ids[self.i];
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
