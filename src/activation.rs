///Represents the actual function of the neuron. This is stateless, unlike
/// the actual neuron.
/// 
/// The ActivationFunction trait describes a functor that provides simple, stateless
/// transformation of the input signal in an ergonomic way. The Activation function is
/// never constructed, and is only accessed in a static context.
/// 
/// ```
/// struct LinearActivation;
/// impl ActivationFunction for LinearActivation {
///     fn activate(f_in: f32) -> f32 {
///         if (f_in < -1.0) {
///             -1.0
///         } else if (f_in > 1.0) {
///             1.0
///         }else {
///             f_in
///         }
///     }
/// }
/// // declaring the ActivationFunction...
/// let layer : ConnectedGenericLayer<_, LinearActivation, SIZE, SIZE_PREV> = ConnectedGenericLayer::new(...);
/// ```
pub trait ActivationFunction {
    fn activate(f_in : f32) -> f32;
}