using Silk.NET.OpenGL;

namespace Revengine.Source.Engine.Render.Textures
{
    public class Texture2D : ITexture
    {
        private readonly GL _context;
        private readonly uint _id;
        private readonly TextureTarget _textureType = TextureTarget.Texture2D;

        internal Texture2D(GL context, uint id)
        {
            _context = context;
            _id = id;
        }

        public void Bind()
        {
            _context.ActiveTexture(TextureUnit.Texture0);
            _context.BindTexture(_textureType, _id);
        }

        public void Unbind()
        {
            _context.BindTexture(_textureType, 0);
        }

        public void Dispose()
        {
            _context.DeleteTexture(_id);
        }
    }
}