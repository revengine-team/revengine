using System.Numerics;


namespace Revengine.Source.Engine.Render.Cameras
{
    public class Camera
    {
        private Vector3 _position;
        private Vector3 _front;
        private Vector3 _up;

        private float _aspectRatio;

        private float _yaw = -90.0f; 
        private float _pitch;

        private float _zoom = 45.0f;

        internal Camera(Vector3 position, Vector3 front, Vector3 up, float aspectRatio)
        {
            _position = position;
            _front = front;
            _up = up;
            _aspectRatio = aspectRatio;
        }

        public Matrix4x4 ViewMatrix()
        {
            return Matrix4x4.CreateLookAt(_position, _position + _front, _up);
        }
    }
}