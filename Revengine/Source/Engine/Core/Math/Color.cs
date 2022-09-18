using System.Numerics;

class Color
{
    public float Red { get; set; } = 0.0f;
    public float Green { get; set; } = 0.0f;
    public float Blue { get; set; } = 0.0f;
    public float Alpha { get; set; } = 1.0f;

    public Color() { }

    public Color(float grey)
    {
        Red = grey;
        Green = grey;
        Blue = grey;
    }

    public Color(float red, float green, float blue)
    {
        Red = red;
        Green = green;
        Blue = blue;
    }

    public Color(float red, float green, float blue, float alpha)
    {
        Red = red;
        Green = green;
        Blue = blue;
        Alpha = alpha;
    }

    public Color(Vector3 color)
    {
        Red = color.X;
        Green = color.Y;
        Blue = color.Z;
    }

    public Color(Vector4 color)
    {
        Red = color.X;
        Green = color.Y;
        Blue = color.Z;
        Alpha = color.W;
    }
}