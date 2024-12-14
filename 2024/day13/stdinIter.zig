const std = @import("std");
pub const Iterator = struct {
    reader: std.io.BufferedReader(4096, std.fs.File.Reader),
    buffer: [4096]u8,
    pub fn next(self: *Iterator) !?[]const u8 {
        while (try self.reader.reader().readUntilDelimiterOrEof(&self.buffer, '\n')) |line| {
            return line;
        }
        return null;
    }
    pub fn new() Iterator {
        return Iterator{ .reader = std.io.bufferedReader(std.io.getStdIn().reader()), .buffer = undefined };
    }
};
pub var stdin: Iterator = Iterator.new();
