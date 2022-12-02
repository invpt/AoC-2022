const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input", .{});
    defer file.close();
    const stdin = file.reader();
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    
    var max3: u64 = 0;
    var max2: u64 = 0;
    var max1: u64 = 0;
    var curr: u64 = 0;
    while (true) {
        var line = std.ArrayList(u8).init(allocator);
        try stdin.readUntilDelimiterArrayList(&line, '\n', 100000);
        if (line.items.len == 0) {
            if (curr > max1) {
                max3 = max2;
                max2 = max1;
                max1 = curr;
            } else if (curr > max2) {
                max3 = max2;
                max2 = curr;
            } else if (curr > max3) {
                max3 = curr;
            }
            curr = 0;
        } else {
            curr += try std.fmt.parseInt(u64, line.items, 10);
        }

        std.debug.print("Current max: {d}\n", .{max1 + max2 + max3});
    }
}