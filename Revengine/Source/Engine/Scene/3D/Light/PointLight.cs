using System.Numerics;

class PointLight
{
    public Color Color { get; set; } = new Color(1.0f);

    /// <summary>
    /// Point light's luminosity Constant parameter:
    /// <para>Luminosity = 1 / (Constant + Linear * Distance + Quadratic * Distance^2)</para>
    /// <para>More info: https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation</para>
    /// </summary>
    public float DistributionConstant { get; set; } = 1.0f;

    /// <summary>
    /// Point light's luminosity Linear parameter:
    /// <para>Luminosity = 1 / (Constant + Linear * Distance + Quadratic * Distance^2)</para>
    /// <para>More info: https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation</para>
    /// </summary>
    public float DistributionLinear { get; set; } = 0.07f;

    /// <summary>
    /// Point light's luminosity Quadratic parameter:
    /// <para>Luminosity = 1 / (Constant + Linear * Distance + Quadratic * Distance^2)</para>
    /// <para>More info: https://wiki.ogre3d.org/tiki-index.php?page=-Point+Light+Attenuation</para>
    /// </summary>
    public float DistributionQuadratic { get; set; } = 0.017f;
}