using System.Numerics;

class DirectionalLight
{
    public Vector3 Direction { get; set; } = new Vector3(0.0f, -1.0f, 0.0f);
    public Color Color { get; set; } = new Color(1.0f);
}