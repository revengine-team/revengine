using System.Numerics;

class CharacterBody : PhysicsBody
{
    public Vector3 Velocity { get; set; } = Vector3.Zero;
    public CharacterBody() { }

    public void MoveAndSlide() { }
}