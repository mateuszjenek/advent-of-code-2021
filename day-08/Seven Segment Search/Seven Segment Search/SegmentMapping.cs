using System.Collections.ObjectModel;

namespace Seven_Segment_Search;

public class SegmentMapping
{
    public char A { get; }
    public char B { get; }
    public char C { get; }
    public char D { get; }
    public char E { get; }
    public char F { get; }
    public char G { get; }

    private SegmentMapping(char a, char b, char c, char d, char e, char f, char g)
    {
        A = a;
        B = b;
        C = c;
        D = d;
        E = e;
        F = f;
        G = g;
    }

    public static SegmentMapping Resolve(List<Digit> digits)
    {
        var a = ResolveASegment(digits);
        var f = ResolveFSegment(digits, a);
        var c = ResolveCSegment(digits, a, f);
        var b = ResolveBSegment(digits, f, c);
        var d = ResolveDSegment(digits, b, c, f);
        var g = ResolveGSegment(digits, a, b, c, d, f);
        var e = ResolveESegment(digits, a, b, c, d, f, g);
        return new SegmentMapping(a, b, c, d, e, f, g);
    }
    
    private static char ResolveASegment(List<Digit> digits)
    {
        Digit? two = null;
        Digit? seven = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 2) two = digit;
            if (digit.Segments.Length is 3) seven = digit;

            if (two != null && seven != null) break;
        }
        
        if (two == null || seven == null) throw new Exception();

        foreach (var segement in seven.Segments)
        {
            if (!two.Segments.Contains(segement)) return segement;
        }

        throw new Exception();
    }

    private static char ResolveFSegment(List<Digit> digits, char aSegment)
    {
        Digit? seven = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 3)
            {
                seven = digit;
                break;
            }
        }
        if (seven == null) throw new Exception();
        
        Digit? six = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 6)
            {
                var containSeven = true;
                foreach (var sevenSegment in seven.Segments)
                    if (!digit.Segments.Contains(sevenSegment))
                    {
                        containSeven = false;
                        break;
                    }

                if (!containSeven)
                {
                    six = digit;
                    break;
                }
            }
        }
        if (six == null) throw new Exception();

        foreach (var sevenSegment in seven.Segments)
        {
            if (sevenSegment != aSegment && six.Segments.Contains(sevenSegment)) return sevenSegment;
        }
        throw new Exception();
    }

    private static char ResolveCSegment(List<Digit> digits, char aSegment, char fSegment)
    {
        Digit? seven = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 3)
            {
                seven = digit;
                break;
            }
        }
        if (seven == null) throw new Exception();
        foreach (var sevenSegment in seven.Segments)
        {
            if (sevenSegment != aSegment && sevenSegment != fSegment) return sevenSegment;
        }
        throw new Exception();
    }
    
    private static char ResolveBSegment(List<Digit> digits, char fSegment, char cSegment)
    {
        Digit? four = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 4)
            {
                four = digit;
                break;
            }
        }
        if (four == null) throw new Exception();

        Digit? three = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 5)
            {
                var containC = digit.Segments.Contains(cSegment);
                var containF = digit.Segments.Contains(fSegment);
                if (containC && containF)
                {
                    three = digit;
                    break;
                }
                
            }
        }
        if (three == null) throw new Exception();
        
        foreach (var fourSegment in four.Segments)
        {
            if (fourSegment != cSegment && fourSegment != fSegment && !three.Segments.Contains(fourSegment)) return fourSegment;
        }
        throw new Exception();
    }
    
    private static char ResolveDSegment(List<Digit> digits, char bSegment, char cSegment, char fSegment)
    {
        Digit? four = null;
        foreach (var digit in digits)
        {
            if (digit.Segments.Length is 4)
            {
                four = digit;
                break;
            }
        }
        if (four == null) throw new Exception();
        foreach (var fourSegment in four.Segments)
        {
            if (fourSegment != bSegment && fourSegment != cSegment && fourSegment != fSegment) return fourSegment;
        }
        throw new Exception();
    }

    private static char ResolveGSegment(List<Digit> digits, char aSegment, char bSegment, char cSegment, char dSegment,
        char fSegment)
    {
        foreach (var digit in digits)
        {
            if (digit.Segments.Length == 6 
                && digit.Segments.Contains(aSegment) 
                && digit.Segments.Contains(bSegment) 
                && digit.Segments.Contains(cSegment) 
                && digit.Segments.Contains(dSegment) 
                && digit.Segments.Contains(fSegment))
            {
                foreach (var segment in digit.Segments)
                {
                    if (segment != aSegment && segment != bSegment && segment != cSegment && segment != dSegment &&
                        segment != fSegment) return segment;
                }
            }
        }
        throw new Exception();
    }
    private static char ResolveESegment(List<Digit> digits, char aSegment, char bSegment, char cSegment, char dSegment,
        char fSegment, char gSegment)
    {
        foreach (var digit in digits)
        {
            if (digit.Segments.Length == 7)
            {
                foreach (var segment in digit.Segments)
                {
                    if (segment != aSegment && segment != bSegment && segment != cSegment && segment != dSegment &&
                        segment != fSegment && segment != gSegment) return segment;
                }
            }
        }
        throw new Exception();
    }
}