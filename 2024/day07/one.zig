const std = @import("std");
const stdin = @import("stdinIter.zig");

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var allocator: std.mem.Allocator = undefined;

const Equation = struct {
    result: i64,
    operands: std.ArrayList(i64),
    fn deinit(self: *Equation) void {
        self.operands.deinit();
    }
    fn is_ok_recursion(self: *Equation, current: i64, _operands: std.ArrayList(i64)) !bool {
        if (current == self.result and _operands.items.len == 0) return true;
        if (current > self.result) return false;
        if (_operands.items.len == 0) return false;
        var operands = try _operands.clone();
        defer operands.deinit();
        errdefer operands.deinit();
        const operand = operands.pop();
        return try self.is_ok_recursion(current * operand, operands) or try self.is_ok_recursion(current + operand, operands);
    }
    fn is_ok(self: *Equation) !bool {
        var operands = try self.operands.clone();
        defer operands.deinit();
        errdefer operands.deinit();
        const operand = operands.pop();
        return try self.is_ok_recursion(operand, operands);
    }
};

pub fn readInput() !std.ArrayList(Equation) {
    var result = std.ArrayList(Equation).init(allocator);
    errdefer _ = {
        for (result.items) |*equation| equation.deinit();
        result.deinit();
    };
    while (try stdin.stdin.next()) |line| {
        var splitColon = std.mem.splitAny(u8, line, ":");
        var equation = Equation{ .result = try std.fmt.parseInt(i64, splitColon.next() orelse "", 10), .operands = std.ArrayList(i64).init(allocator) };
        errdefer equation.deinit();
        var splitSpaces = std.mem.splitAny(u8, splitColon.next() orelse "", " ");
        while (splitSpaces.next()) |operand| {
            const intVal = std.fmt.parseInt(i64, operand, 10) catch {
                continue;
            };
            try equation.operands.insert(0, intVal);
        }
        try result.append(equation);
    }
    return result;
}

pub fn main() !void {
    defer _ = gpa.deinit();
    allocator = gpa.allocator();
    var input: std.ArrayList(Equation) = try readInput();
    defer _ = {
        for (input.items) |*equation| equation.deinit();
        input.deinit();
    };
    var result: i64 = 0;
    for (input.items) |*equation| {
        if (try equation.is_ok()) result += equation.result;
    }
    std.debug.print("{any}\n", .{result});
}
