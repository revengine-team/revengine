using Revengine.Source.Engine.Render.Cameras;
using System.Numerics;

class Camera3D
{
    private Camera camera = new Camera(
        Vector3.Zero,
        new Vector3(0.0f, 0.0f, -1.0f),
        new Vector3(0.0f, 1.0f, 0.0f),
        10.0f
    );

    public Camera3D() { }
}