using Revengine.Source.Engine.Render.Buffers.Vertices;
using Revengine.Source.Engine.Render.Textures;

namespace Revengine.Source.Engine.Render.Meshes
{
    public class MeshData
    {
        private readonly IList<Vertex> _vertices;
        private readonly IList<uint> _indices;
        private readonly IList<ITexture> _textures;

        public MeshData(IList<Vertex> vertices, IList<uint> indices, IList<ITexture> textures)
        {
            _vertices = vertices;
            _indices = indices;
            _textures = textures;
        }
    }
}