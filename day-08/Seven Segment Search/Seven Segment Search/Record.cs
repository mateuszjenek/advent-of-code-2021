namespace Seven_Segment_Search;

public class Record
{
    public List<Digit> Signals = new List<Digit>();
    public List<Digit> Output = new List<Digit>();

    public Record(string line)
    {
        var data = line.Split(" | ");
        foreach (var segments in data[0].Split(' '))
        {
            Signals.Add(new Digit(segments));
        }

        foreach (var segments in data[1].Split(' '))
        {
            Output.Add(new Digit(segments));
        }
    }

    public int GetOutput(SegmentMapping mapping)
    {
        var output = new List<Digit>(Output);
        output.Reverse();

        var result = 0.0;
        for (var index = 0; index < output.Count; index++)
        {
            result += output[index].ToInt(mapping) * Math.Pow(10, index);
        }

        return (int) result;
    }
}