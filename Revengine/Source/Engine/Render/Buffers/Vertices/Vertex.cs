using System.Numerics;

namespace Revengine.Source.Engine.Render.Buffers.Vertices
{
    public struct Vertex
    {
        public Vector3 Position { get; }
        public Vector3 Normal { get; }
        public Vector3 Color { get; }
        public Vector2 TexturesUV { get; }

        internal Vertex(Vector3 position, Vector3 normal, Vector3 color, Vector2 texturesUV)
        {
            Position = position;
            Normal = normal;
            Color = color;
            TexturesUV = texturesUV;
        }
    }
}