namespace Seven_Segment_Search;

public class Digit
{
    public string Segments { get; }

    public Digit(string segments)
    {
        var chars = segments.OrderBy(segment => segment);
        this.Segments = String.Concat(chars);;
    }

    public int ToInt(SegmentMapping mapping)
    {
        if (isZero(mapping)) return 0;
        if (isOne(mapping)) return 1;
        if (isTwo(mapping)) return 2;
        if (isThree(mapping)) return 3;
        if (isFour(mapping)) return 4;
        if (isFive(mapping)) return 5;
        if (isSix(mapping)) return 6;
        if (isSeven(mapping)) return 7;
        if (isEight(mapping)) return 8;
        if (isNine(mapping)) return 9;
        throw new Exception();
    }

    private bool isZero(SegmentMapping mapping)
    {
        return Segments.Length == 6
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.B)
               && Segments.Contains(mapping.C)
               && Segments.Contains(mapping.E)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.G);
    }
    
    private bool isOne(SegmentMapping mapping)
    {
        return Segments.Length == 2;
    }
    
    private bool isTwo(SegmentMapping mapping)
    {
        return Segments.Length == 5
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.C)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.E)
               && Segments.Contains(mapping.G);
    }
    
    private bool isThree(SegmentMapping mapping)
    {
        return Segments.Length == 5
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.C)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.G);
    }
    
    private bool isFour(SegmentMapping mapping)
    {
        return Segments.Length == 4;
    }
    
    private bool isFive(SegmentMapping mapping)
    {
        return Segments.Length == 5
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.B)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.G);
    }
    
    private bool isSix(SegmentMapping mapping)
    {
        return Segments.Length == 6
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.B)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.E)
               && Segments.Contains(mapping.G);
    }
    
    private bool isSeven(SegmentMapping mapping)
    {
        return Segments.Length == 3;
    }
    
    private bool isEight(SegmentMapping mapping)
    {
        return Segments.Length == 7
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.B)
               && Segments.Contains(mapping.C)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.E)
               && Segments.Contains(mapping.G);
    }
    
    private bool isNine(SegmentMapping mapping)
    {
        return Segments.Length == 6
               && Segments.Contains(mapping.A)
               && Segments.Contains(mapping.B)
               && Segments.Contains(mapping.C)
               && Segments.Contains(mapping.D)
               && Segments.Contains(mapping.F)
               && Segments.Contains(mapping.G);
    }
}