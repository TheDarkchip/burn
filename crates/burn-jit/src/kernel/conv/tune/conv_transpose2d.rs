use burn_tensor::{ops::ConvTransposeOptions, ElementConversion, Shape};
use cubecl::tune;
use cubecl::tune::{local_tuner, tune_with, LocalTuner};

use crate::kernel::conv::conv_transpose2d;
use crate::kernel::conv::ConvTranspose2dStrategy;
use crate::{
    kernel::prng::random_uniform, tensor::JitTensor, FloatElement, JitAutotuneKey, JitRuntime,
    JitTuneId,
};

use super::ConvTranspose2dAutotuneKey;

/// Executes autotune on conv2d operations
pub fn conv_transpose2d_autotune<R: JitRuntime, E: FloatElement>(
    input: JitTensor<R>,
    weights: JitTensor<R>,
    bias: Option<JitTensor<R>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R> {
    let client = input.client.clone();

    static TUNER: LocalTuner<JitAutotuneKey, JitTuneId> = local_tuner!();

    TUNER.execute(
        &JitTuneId::new::<R>(&input.device),
        &client,
        Box::new(ConvTranspose2dOperations::<R, E>::new(
            input, weights, bias, options,
        )),
    )
}

#[tune(operations(conv_transpose2d_direct, conv_transpose2d_col2im), create_key = create_key::<R, E>, should_run = should_run)]
fn conv_transpose2d_operations<R: JitRuntime, E: FloatElement>(
    key: JitAutotuneKey,
    input: JitTensor<R>,
    weights: JitTensor<R>,
    bias: Option<JitTensor<R>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R> {
    let key = match key {
        JitAutotuneKey::ConvTranspose2d(key) => key,
        _ => unreachable!(),
    };
    let device = &input.device;

    let random_bounds: (E, E) = ((-1.0).elem::<E>(), (1.0).elem::<E>());
    let input_shape = Shape::new([key.batch_size, key.in_channels, key.height, key.width]);
    let input = random_uniform(input_shape, device, random_bounds.0, random_bounds.1);
    let c_per_grp = key.in_channels / key.groups;
    let [kernel_h, kernel_w] = key.kernel_size;
    let weight_shape = Shape::new([key.out_channels, c_per_grp, kernel_h, kernel_w]);
    let weights = random_uniform(weight_shape, device, random_bounds.0, random_bounds.1);
    let bias_shape = Shape::new([key.out_channels]);
    let bias = key
        .has_bias
        .then(|| random_uniform(bias_shape, device, random_bounds.0, random_bounds.1));

    tune_with!(input, weights, bias, options)
}

fn create_key<R: JitRuntime, E: FloatElement>(
    input: &JitTensor<R>,
    weights: &JitTensor<R>,
    bias: &Option<JitTensor<R>>,
    options: &ConvTransposeOptions<2>,
) -> JitAutotuneKey {
    let [batch_size, in_channels, height, width] = input.shape.dims();
    let [out_channels, _, kernel_h, kernel_w] = weights.shape.dims();
    let ConvTransposeOptions {
        stride,
        padding,
        dilation,
        groups,
        padding_out,
    } = options.clone();
    JitAutotuneKey::ConvTranspose2d(ConvTranspose2dAutotuneKey::new(
        [kernel_h, kernel_w],
        stride,
        padding,
        padding_out,
        dilation,
        groups,
        in_channels,
        out_channels,
        height,
        width,
        batch_size,
        bias.is_some(),
        E::dtype(),
    ))
}

fn should_run<R: JitRuntime, F: FloatElement>(
    _op: &ConvTranspose2dOperations<R, F>,
    key: &JitAutotuneKey,
    index: usize,
) -> bool {
    let key = match key {
        JitAutotuneKey::ConvTranspose2d(key) => key,
        _ => unreachable!(),
    };

    match index {
        // im2col
        1 => cubecl::convolution::conv2d::batches_per_run(key.batch_size, key.height, key.width)
            .is_some(),
        _ => true,
    }
}

fn conv_transpose2d_direct<R: JitRuntime, E: FloatElement>(
    input: JitTensor<R>,
    weights: JitTensor<R>,
    bias: Option<JitTensor<R>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R> {
    conv_transpose2d::conv_transpose2d::<R, E>(
        input,
        weights,
        bias,
        options,
        ConvTranspose2dStrategy::Direct,
    )
}

fn conv_transpose2d_col2im<R: JitRuntime, E: FloatElement>(
    input: JitTensor<R>,
    weights: JitTensor<R>,
    bias: Option<JitTensor<R>>,
    options: ConvTransposeOptions<2>,
) -> JitTensor<R> {
    conv_transpose2d::conv_transpose2d::<R, E>(
        input,
        weights,
        bias,
        options,
        ConvTranspose2dStrategy::Gemm,
    )
}
