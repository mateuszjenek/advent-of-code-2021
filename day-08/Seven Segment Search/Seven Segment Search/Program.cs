using Seven_Segment_Search;

var result = 0;

foreach (var line in File.ReadLines("/Users/mateuszjenek/Desktop/advent-of-code-2021/day-08/input.txt"))
{
    var record = new Record(line);
    var allDigits = new List<Digit>(record.Signals);
    allDigits.AddRange(record.Output);
    var mapping = SegmentMapping.Resolve(allDigits);
    result += record.GetOutput(mapping);
}

Console.Out.WriteLine(result);





