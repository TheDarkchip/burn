use alloc::vec::Vec;

use crate::{alloc::borrow::ToOwned, cast::ToElement};

use crate::{
    Bool, Distribution, Element, ElementConversion, Int, Shape, Tensor, backend::Backend, check,
    check::TensorCheck,
};

use super::Numeric;

/// Default RTOL value for `is_close` and `all_close`.
pub const DEFAULT_RTOL: f64 = 1e-5;

/// Default ATOL value for `is_close` and `all_close`.
pub const DEFAULT_ATOL: f64 = 1e-8;

impl<B, const D: usize, K> Tensor<B, D, K>
where
    B: Backend,
    K: Numeric<B>,
    K::Elem: Element,
{
    /// Applies element wise addition operation.
    ///
    /// `y = x2 + x1`
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to add.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1 + tensor2;
    ///    println!("{tensor}");
    ///    // [[3.0, 1.0, 7.0], [6.0, 11.0, 9.0]]
    /// }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Self) -> Self {
        check!(TensorCheck::binary_ops_ew("Add", &self, &other));
        Self::new(K::add(self.primitive, other.primitive))
    }

    /// Applies element wise addition operation with a scalar.
    ///
    /// `y = x + s`
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to add, element wise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let scalar = 2.0;
    ///   let tensor = tensor + scalar;
    ///   println!("{tensor}");
    ///   // [[3.0, 0.0, 5.0], [7.0, 11.0, 8.0]]
    /// }
    /// ```
    pub fn add_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::add_scalar::<E>(self.primitive, other))
    }

    /// Applies element wise subtraction operation.
    ///
    /// `y = x2 - x1`
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to subtract.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///   let tensor = tensor1 - tensor2;
    ///   println!("{tensor}");
    ///   // [[-1.0, -5.0, -1.0], [4.0, 7.0, 3.0]]
    /// }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Self) -> Self {
        check!(TensorCheck::binary_ops_ew("Sub", &self, &other));
        Self::new(K::sub(self.primitive, other.primitive))
    }

    /// Applies element wise subtraction operation with a scalar.
    ///
    /// `y = x - s`
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to subtract, element wise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let scalar = 2.0;
    ///    let tensor = tensor - scalar;
    ///    println!("{tensor}");
    ///    // [[-1.0, -4.0, 1.0], [3.0, 7.0, 4.0]]
    /// }
    /// ```
    pub fn sub_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::sub_scalar::<E>(self.primitive, other))
    }

    /// Applies element wise division operation.
    ///
    /// `y = x2 / x1`
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to divide.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1 / tensor2;
    ///    println!("{tensor}");
    ///    // [[0.5, -0.6666667, 0.75], [5.0, 4.5, 2.0]]
    /// }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, other: Self) -> Self {
        check!(TensorCheck::binary_ops_ew("Div", &self, &other));
        Self::new(K::div(self.primitive, other.primitive))
    }

    /// Applies element wise division operation with a scalar.
    ///
    /// `y = x / s`
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to divide, element wise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let scalar = 2.0;
    ///    let tensor = tensor / scalar;
    ///    println!("{tensor}");
    ///    // [[0.5, -1.0, 1.5], [2.5, 4.5, 3.0]]
    /// }
    /// ```
    pub fn div_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::div_scalar::<E>(self.primitive, other))
    }

    /// Applies element wise the remainder operation with a scalar.
    ///
    /// `y = x2 % x1`
    pub fn remainder(self, other: Self) -> Self {
        Self::new(K::remainder(self.primitive, other.primitive))
    }

    /// Applies element wise the remainder operation with a scalar.
    ///
    /// `y = x % s`
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to divide, element wise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let scalar = 2.0;
    ///    let tensor = tensor1 % scalar;
    ///    println!("{tensor}");
    ///    // [[1.0, 0.0, 1.0], [1.0, 1.0, 0.0]]
    /// }
    /// ```
    pub fn remainder_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::remainder_scalar::<E>(self.primitive, other))
    }

    /// Applies element wise multiplication operation.
    ///
    /// `y = x2 * x1`
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to multiply.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1 * tensor2;
    ///    println!("{tensor}");
    ///    // [[2.0, -6.0, 12.0], [5.0, 18.0, 18.0]]
    /// }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Self) -> Self {
        check!(TensorCheck::binary_ops_ew("Mul", &self, &other));
        Self::new(K::mul(self.primitive, other.primitive))
    }

    /// Applies element wise multiplication operation with a scalar.
    ///
    /// `y = x * s`
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to multiply, element wise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let scalar = 2.0;
    ///    let tensor = tensor * scalar;
    ///    println!("{tensor}");
    ///    // [[2.0, -4.0, 6.0], [10.0, 18.0, 12.0]]
    /// }
    /// ```
    pub fn mul_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::mul_scalar::<E>(self.primitive, other))
    }

    /// Switch sign of each element in the tensor.
    ///
    /// `y = -x`
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = -tensor;
    ///    println!("{tensor}");
    ///    // [[-1.0, 2.0, -3.0], [-5.0, -9.0, -6.0]]
    /// }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn neg(self) -> Self {
        Self::new(K::neg(self.primitive))
    }

    /// Returns the signs of the elements of the input tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.sign();
    ///    println!("{tensor}");
    ///    // [[1.0, -1.0, 1.0], [1.0, 1.0, 1.0]]
    /// }
    /// ```
    pub fn sign(self) -> Self {
        Self::new(K::sign(self.primitive))
    }

    /// Create a tensor of the given shape where each element is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::zeros(Shape::new([2, 3]), &device);
    ///    println!("{tensor}");
    ///    // [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
    /// }
    /// ```
    pub fn zeros<S: Into<Shape>>(shape: S, device: &B::Device) -> Self {
        let shape = shape.into();
        check!(TensorCheck::creation_ops::<D>("Zeros", &shape.dims));
        Self::new(K::zeros(shape, device))
    }

    /// Returns a new tensor with the same shape and device as the current tensor filled with zeros.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.zeros_like();
    ///   println!("{tensor}");
    ///   // [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
    /// }
    /// ```
    pub fn zeros_like(&self) -> Self {
        Self::zeros(self.shape(), &self.device())
    }

    /// Create a tensor of the given shape where each element is one.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::ones(Shape::new([2, 3]), &device);
    ///   println!("{tensor}");
    ///   // [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
    /// }
    /// ```
    pub fn ones<S: Into<Shape>>(shape: S, device: &B::Device) -> Self {
        let shape = shape.into();
        check!(TensorCheck::creation_ops::<D>("Ones", &shape.dims));
        Self::new(K::ones(shape, device))
    }

    /// Returns a new tensor with the same shape and device as the current tensor filled with ones.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.ones_like();
    ///    println!("{tensor}");
    ///    // [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]
    /// }
    /// ```
    pub fn ones_like(&self) -> Self {
        Self::ones(self.shape(), &self.device())
    }

    /// Create a tensor of the given shape where each element is equal to the provided value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::full(Shape::new([2, 3]), 5.0, &device);
    ///   println!("{tensor}");
    ///   // [[5.0, 5.0, 5.0], [5.0, 5.0, 5.0]]
    /// }
    /// ```
    pub fn full<S: Into<Shape>, E: ElementConversion>(
        shape: S,
        fill_value: E,
        device: &B::Device,
    ) -> Self {
        let shape = shape.into();
        check!(TensorCheck::creation_ops::<D>("Full", &shape.dims));
        Self::new(K::full(shape, fill_value, device))
    }

    ///Returns a new tensor with the same shape and device as the current tensor filled with the provided value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.full_like(5.0);
    ///    println!("{tensor}");
    ///    // [[5.0, 5.0, 5.0], [5.0, 5.0, 5.0]]
    /// }
    /// ```
    pub fn full_like<E: ElementConversion>(&self, fill_value: E) -> Self {
        Self::full(self.shape(), fill_value, &self.device())
    }

    /// Aggregate all elements in the tensor with the mean operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.mean();
    ///    println!("{tensor}");
    ///    // [3.6666667]
    /// }
    /// ```
    pub fn mean(self) -> Tensor<B, 1, K> {
        Tensor::new(K::mean(self.primitive))
    }

    /// Aggregate all elements in the tensor with the sum operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.sum();
    ///   println!("{tensor}");
    ///   // [22.0]
    /// }
    /// ```
    pub fn sum(self) -> Tensor<B, 1, K> {
        Tensor::new(K::sum(self.primitive))
    }

    /// Aggregate all elements along the given *dimension* or *axis*
    /// in the tensor with the mean operation.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension or axis along which to aggregate the elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.clone().mean_dim(0);
    ///   println!("{tensor}");
    ///   // [[3.0, 3.5, 4.5]]
    ///   let tensor = tensor.clone().mean_dim(1);
    ///   println!("{tensor}");
    ///   // [[0.6666667], [6.6666665]]
    /// }
    /// ```
    pub fn mean_dim(self, dim: usize) -> Self {
        check!(TensorCheck::aggregate_dim::<D>("Mean", dim));
        Self::new(K::mean_dim(self.primitive, dim))
    }

    /// Aggregate all elements along the given *dimension* or *axis*
    /// in the tensor with the sum operation.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension or axis along which to aggregate the elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.clone().sum_dim(0);
    ///    println!("{tensor}");
    ///    // [[6.0, 7.0, 9.0]]
    ///    let tensor = tensor.clone().sum_dim(1);
    ///    println!("{tensor}");
    ///    // [[2.0], [20.0]]
    /// }
    /// ```
    pub fn sum_dim(self, dim: usize) -> Self {
        check!(TensorCheck::aggregate_dim::<D>("Sum", dim));
        Self::new(K::sum_dim(self.primitive, dim))
    }

    /// Aggregate all elements in the tensor with the product operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.prod();
    ///    println!("{tensor}");
    ///    // [-1620.0]
    /// }
    /// ```
    pub fn prod(self) -> Tensor<B, 1, K> {
        Tensor::new(K::prod(self.primitive))
    }

    /// Aggregate all elements along the given *dimension* or *axis*
    /// in the tensor with the product operation.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension or axis along which to aggregate the elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.clone().prod_dim(0);
    ///    println!("{tensor}");
    ///    // [[5.0, -18.0, 18.0]]
    ///    let tensor = tensor.clone().prod_dim(1);
    ///    println!("{tensor}");
    ///    // [[-6.0], [270.0]]
    /// }
    /// ```
    pub fn prod_dim(self, dim: usize) -> Self {
        check!(TensorCheck::aggregate_dim::<D>("Prod", dim));
        Self::new(K::prod_dim(self.primitive, dim))
    }

    /// Applies element wise equal comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.equal_elem(3.0);
    ///    println!("{tensor}");
    ///    // [[false, false, true], [false, false, false]]
    /// }
    /// ```
    pub fn equal_elem<E: Element>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::equal_elem(self.primitive, other.elem()))
    }

    /// Applies element wise non-equality comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.not_equal_elem(3.0);
    ///    println!("{tensor}");
    ///    // [[true, true, false], [true, true, true]]
    /// }
    /// ```
    pub fn not_equal_elem<E: Element>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::not_equal_elem(self.primitive, other.elem()))
    }

    /// Applies element wise greater comparison and returns a boolean tensor.
    ///
    /// # Panics
    ///
    /// If the two tensors don't have the same shape.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor2 = Tensor::<B, 2>::from_data([[1.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///   let tensor = tensor1.greater(tensor2);
    ///   println!("{tensor}");
    ///   // [[false, false, false], [true, true, true]]
    /// }
    /// ```
    pub fn greater(self, other: Self) -> Tensor<B, D, Bool> {
        check!(TensorCheck::binary_ops_ew("Greater", &self, &other));
        Tensor::new(K::greater(self.primitive, other.primitive))
    }

    /// Applies element wise greater-equal comparison and returns a boolean tensor.
    ///
    /// # Panics
    ///
    /// If the two tensors don't have the same shape.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[1.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.greater_equal(tensor2);
    ///    println!("{tensor}");
    ///    // [[true, false, false], [true, true, true]]
    /// }
    /// ```
    pub fn greater_equal(self, other: Self) -> Tensor<B, D, Bool> {
        check!(TensorCheck::binary_ops_ew("Greater_equal", &self, &other));
        Tensor::new(K::greater_equal(self.primitive, other.primitive))
    }

    /// Applies element wise lower comparison and returns a boolean tensor.
    ///
    /// # Panics
    ///
    /// If the two tensors don't have the same shape.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[1.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.lower(tensor2);
    ///    println!("{tensor}");
    ///    // [[false, true, true], [false, false, false]]
    /// }
    /// ```
    pub fn lower(self, other: Self) -> Tensor<B, D, Bool> {
        check!(TensorCheck::binary_ops_ew("Lower", &self, &other));
        Tensor::new(K::lower(self.primitive, other.primitive))
    }

    /// Applies element wise lower-equal comparison and returns a boolean tensor.
    ///
    /// # Panics
    ///
    /// If the two tensors don't have the same shape.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[1.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.lower_equal(tensor2);
    ///    println!("{tensor}");
    ///    // [[true, true, true], [false, false, false]]
    /// }
    /// ```
    pub fn lower_equal(self, other: Self) -> Tensor<B, D, Bool> {
        check!(TensorCheck::binary_ops_ew("Lower_equal", &self, &other));
        Tensor::new(K::lower_equal(self.primitive, other.primitive))
    }

    /// Applies greater than `other` comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.greater_elem(3.0);
    ///    println!("{tensor}");
    ///    // [[false, false, true], [true, true, true]]
    /// }
    /// ```
    pub fn greater_elem<E: ElementConversion>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::greater_elem(self.primitive, other.elem()))
    }

    /// Applies greater-equal than `other` comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.greater_equal_elem(3.0);
    ///    println!("{tensor}");
    ///    // [[false, false, true], [true, true, true]]
    /// }
    /// ```
    pub fn greater_equal_elem<E: ElementConversion>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::greater_equal_elem(self.primitive, other.elem()))
    }

    /// Applies lower than `other` comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///     let device = B::Device::default();
    ///     let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///     let tensor = tensor.lower_elem(3.0);
    ///     println!("{tensor}");
    ///     // [[true, true, false], [false, false, false]]
    /// }
    /// ```
    pub fn lower_elem<E: ElementConversion>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::lower_elem(self.primitive, other.elem()))
    }

    /// Applies lower-equal than `other` comparison and returns a boolean tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - The element to compare.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.lower_equal_elem(3.0);
    ///    println!("{tensor}");
    ///    // [[true, true, true], [false, false, false]]
    /// }
    /// ```
    pub fn lower_equal_elem<E: ElementConversion>(self, other: E) -> Tensor<B, D, Bool> {
        Tensor::new(K::lower_equal_elem(self.primitive, other.elem()))
    }

    /// Update the given tensor with the value tensor where the mask is true.
    ///
    /// This is similar to [mask_fill](Tensor::mask_fill), however the value is a tensor instead of
    /// a scalar.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Bool};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let mask = Tensor::<B, 2, Bool>::from_data([[true, false, true], [false, true, false]], &device);
    ///   let value = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///   let tensor = tensor.mask_where(mask, value);
    ///   println!("{tensor}");
    ///   // [[2.0, -2.0, 4.0], [5.0, 2.0, 6.0]]
    /// }
    /// ```
    pub fn mask_where(self, mask: Tensor<B, D, Bool>, value: Self) -> Self {
        Self::new(K::mask_where(
            self.primitive,
            mask.primitive,
            value.primitive,
        ))
    }

    /// Update the given tensor with the value where the mask is true.
    ///
    /// This is similar to [mask_where](Tensor::mask_where), however the value is a scalar instead of
    /// a tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Bool};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let mask = Tensor::<B, 2, Bool>::from_data([[true, false, true], [false, true, false]], &device);
    ///   let tensor = tensor.mask_fill(mask, 3.0);
    ///   println!("{tensor}");
    ///   // [[3.0, -2.0, 3.0], [5.0, 3.0, 6.0]]
    /// }
    /// ```
    pub fn mask_fill<E: ElementConversion>(self, mask: Tensor<B, D, Bool>, value: E) -> Self {
        Self::new(K::mask_fill(self.primitive, mask.primitive, value.elem()))
    }

    /// Gather tensor elements corresponding to the given indices from the specified dim.
    ///
    /// Example using a 3D tensor:
    ///
    /// `output[i, j, k] = input[indices[i, j, k], j, k]; // dim = 0`
    /// `output[i, j, k] = input[i, indices[i, j, k], k]; // dim = 1`
    /// `output[i, j, k] = input[i, j, indices[i, j, k]]; // dim = 2`
    ///
    /// # Notes
    ///
    /// The index tensor should have the same shape as the original tensor except for the dim
    /// specified.
    ///
    /// # Warning
    /// Not all backends have runtime bound checks for the indices, so make sure the they are valid.
    /// Otherwise, out of bounds indices could lead to unexpected results instead of panicking.
    pub fn gather(self, dim: usize, indices: Tensor<B, D, Int>) -> Self {
        check!(TensorCheck::gather::<D>(
            dim,
            &self.shape(),
            &indices.shape()
        ));

        Self::new(K::gather(dim, self.primitive, indices.primitive))
    }

    /// Assign the gathered elements corresponding to the given indices along the specified dimension
    /// from the value tensor to the original tensor using sum reduction.
    ///
    /// Example using a 3D tensor:
    ///
    /// `input[indices[i, j, k], j, k] += values[i, j, k]; // dim = 0`
    /// `input[i, indices[i, j, k], k] += values[i, j, k]; // dim = 1`
    /// `input[i, j, indices[i, j, k]] += values[i, j, k]; // dim = 2`
    ///
    /// # Notes
    ///
    /// The index tensor should have the same shape as the original tensor except for the specified
    /// dimension. The value and index tensors should have the same shape.
    ///
    /// Other references to the input tensor will not be modified by this operation.
    ///
    /// # Warning
    /// Not all backends have runtime bound checks for the indices, so make sure the they are valid.
    /// Otherwise, out of bounds indices could lead to unexpected results instead of panicking.
    pub fn scatter(self, dim: usize, indices: Tensor<B, D, Int>, values: Self) -> Self {
        check!(TensorCheck::scatter::<D>(
            dim,
            &self.shape(),
            &indices.shape(),
            &values.shape()
        ));

        Self::new(K::scatter(
            dim,
            self.primitive,
            indices.primitive,
            values.primitive,
        ))
    }

    /// Select the tensor elements along the given dimension corresponding to the given indices.
    ///
    /// Example using a 3D tensor:
    ///
    /// `output[i, j, k] = input[indices[i], j, k]; // dim = 0`
    /// `output[i, j, k] = input[i, indices[j], k]; // dim = 1`
    /// `output[i, j, k] = input[i, j, indices[k]]; // dim = 2`
    ///
    /// # Warning
    /// Not all backends have runtime bound checks for the indices, so make sure the they are valid.
    /// Otherwise, out of bounds indices could lead to unexpected results instead of panicking.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Int};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 3>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let indices = Tensor::<B, 1, Int>::from_data([0], &device);
    ///   let tensor = tensor.select(0, indices);
    ///   println!("{tensor}");
    ///   //  [[1.0, -2.0, 3.0]]
    /// }
    /// ```
    pub fn select(self, dim: usize, indices: Tensor<B, 1, Int>) -> Self {
        check!(TensorCheck::select::<D>(dim));
        Self::new(K::select(self.primitive, dim, indices.primitive))
    }

    /// Assign the selected elements along the given dimension corresponding to the given indices
    /// from the value tensor to the original tensor using sum reduction.
    ///
    /// Example using a 3D tensor:
    ///
    /// `input[indices[i], j, k] += values[i, j, k]; // dim = 0`
    /// `input[i, indices[j], k] += values[i, j, k]; // dim = 1`
    /// `input[i, j, indices[k]] += values[i, j, k]; // dim = 2`
    ///
    /// # Warning
    /// Not all backends have runtime bound checks for the indices, so make sure the they are valid.
    /// Otherwise, out of bounds indices could lead to unexpected results instead of panicking.
    pub fn select_assign(
        self,
        dim: usize,
        indices: Tensor<B, 1, Int>,
        values: Tensor<B, D, K>,
    ) -> Self {
        check!(TensorCheck::select_assign::<D>(dim));

        Self::new(K::select_assign(
            self.primitive,
            dim,
            indices.primitive,
            values.primitive,
        ))
    }

    /// Applies the argmax function along the given dimension and returns an integer tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///     let device = B::Device::default();
    ///     let tensor = Tensor::<B, 3>::ones(Shape::new([2, 3, 3]), &device);
    ///     let tensor = tensor.argmax(1);
    ///     println!("{:?}", tensor.shape());
    ///     // Shape { dims: [2, 1, 3] }
    /// }
    /// ```
    pub fn argmax(self, dim: usize) -> Tensor<B, D, Int> {
        Tensor::new(K::argmax(self.primitive, dim))
    }

    /// Find the maximum value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.max();
    ///   println!("{tensor}");
    ///   // [9.0]
    /// }
    /// ```
    pub fn max(self) -> Tensor<B, 1, K> {
        Tensor::new(K::max(self.primitive))
    }

    /// Find the maximum value along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.max_dim(0);
    ///   println!("{tensor}");
    ///   // [[5.0, 9.0, 6.0]]
    /// }
    /// ```
    pub fn max_dim(self, dim: usize) -> Tensor<B, D, K> {
        check!(TensorCheck::aggregate_dim::<D>("Max", dim));

        Tensor::new(K::max_dim(self.primitive, dim))
    }

    /// Find the maximum value along the given dimension.
    ///
    /// Also returns the indices.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let (tensor, index) = tensor.max_dim_with_indices(0);
    ///    // [[5.0, 9.0, 6.0]]
    ///    println!("{tensor}");
    ///    // [[1, 1, 1]]
    ///    println!("{index}");
    /// }
    /// ```
    pub fn max_dim_with_indices(self, dim: usize) -> (Tensor<B, D, K>, Tensor<B, D, Int>) {
        check!(TensorCheck::aggregate_dim::<D>("Max", dim));

        let (tensor, index) = K::max_dim_with_indices(self.primitive, dim);

        let tensor = Tensor::new(tensor);
        let index = Tensor::new(index);

        (tensor, index)
    }

    /// Finds the maximum pair wise values with another tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - Other tensor to find maximum elements with
    ///
    /// # Returns
    ///
    /// A tensor with the same shape as the input tensors containing the maximum value found
    /// in the input tensors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.max_pair(tensor2);
    ///    println!("{tensor}");
    ///    // [[2.0, 3.0, 4.0], [5.0, 9.0, 6.0]]
    /// }
    /// ```
    pub fn max_pair(self, other: Self) -> Self {
        let mask = self.clone().lower(other.clone());
        self.mask_where(mask, other)
    }

    /// Find the maximum absolute value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -7.0, 3.0], [5.0, -1.0, 6.0]], &device);
    ///   let tensor = tensor.max_abs();
    ///   println!("{tensor}");
    ///   // [7.0]
    /// }
    /// ```
    pub fn max_abs(self) -> Tensor<B, 1, K> {
        Tensor::new(K::max_abs(self.primitive))
    }

    /// Find the maximum absolute value along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.max_dim(0);
    ///   println!("{tensor}");
    ///   // [[5.0, 9.0, 6.0]]
    /// }
    /// ```
    pub fn max_abs_dim(self, dim: usize) -> Tensor<B, D, K> {
        check!(TensorCheck::aggregate_dim::<D>("MaxAbs", dim));

        Tensor::new(K::max_abs_dim(self.primitive, dim))
    }

    /// Applies the argmin function along the given dimension and returns an integer tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///     let device = Default::default();
    ///     let tensor = Tensor::<B, 3>::ones(Shape::new([2, 3, 3]), &device);
    ///     let tensor = tensor.argmin(1);
    ///     println!("{:?}", tensor.shape());
    ///     // Shape { dims: [2, 1, 3] }
    /// }
    /// ```
    pub fn argmin(self, dim: usize) -> Tensor<B, D, Int> {
        Tensor::new(K::argmin(self.primitive, dim))
    }

    /// Find the minimum value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.min();
    ///    println!("{tensor}");
    ///    // [-2.0]
    /// }
    /// ```
    pub fn min(self) -> Tensor<B, 1, K> {
        Tensor::new(K::min(self.primitive))
    }

    /// Find the minimum value along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.min_dim(0);
    ///    println!("{tensor}");
    ///    // [[1.0, -2.0, 3.0]]
    /// }
    /// ```
    pub fn min_dim(self, dim: usize) -> Tensor<B, D, K> {
        check!(TensorCheck::aggregate_dim::<D>("Min", dim));
        Tensor::new(K::min_dim(self.primitive, dim))
    }

    /// Find the minimum value along the given dimension.
    ///
    /// Also returns the indices.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[7.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let (tensor, index) = tensor.min_dim_with_indices(0);
    ///    println!("{tensor}");
    ///    // [[5.0, -2.0, 3.0]]
    ///    println!("{}", index);
    ///    // [[1, 0, 0]]
    /// }
    /// ```
    pub fn min_dim_with_indices(self, dim: usize) -> (Tensor<B, D, K>, Tensor<B, D, Int>) {
        check!(TensorCheck::aggregate_dim::<D>("Min", dim));

        let (tensor, index) = K::min_dim_with_indices(self.primitive, dim);

        let tensor = Tensor::new(tensor);
        let index = Tensor::new(index);

        (tensor, index)
    }

    /// Finds the minimum pair wise values with another tensor.
    ///
    /// # Arguments
    ///
    /// * `other` - Other tensor to find minimum elements with
    ///
    /// # Returns
    ///
    /// A tensor with the same shape as the input tensors containing the minimum value found
    /// between each element of the two source tensors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.min_pair(tensor2);
    ///    println!("{tensor}");
    ///    // [[1.0, -2.0, 3.0], [1.0, 2.0, 3.0]]
    /// }
    pub fn min_pair(self, other: Self) -> Self {
        let mask = other.clone().lower(self.clone());
        self.mask_where(mask, other)
    }

    /// Clamp element wise between the given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value.
    /// * `max` - The maximum value.
    ///
    /// # Returns
    ///
    /// A new tensor with the values clamped between the given min and max values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///   let device = Default::default();
    ///   let tensor = Tensor::<B, 2, Int>::from_ints(
    ///    [
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    ///    ],
    ///    &device);
    ///    let tensor = tensor.clamp(2, 6);
    ///    println!("{tensor}");
    ///    // [[2, 2, 3], [4, 5, 6], [6, 6, 6]]
    /// }
    /// ```
    pub fn clamp<E: ElementConversion>(self, min: E, max: E) -> Self {
        Self::new(K::clamp(self.primitive, min.elem(), max.elem()))
    }

    /// Clamp element wise under a minimum value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to clamp.
    /// * `min` - The minimum value.
    ///
    /// # Returns
    ///
    /// A new tensor with the values clamped under the given min value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///    let device = Default::default();
    ///    let tensor = Tensor::<B, 2, Int>::from_ints(
    ///    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    ///    &device);
    ///    let tensor = tensor.clamp_min(4);
    ///    println!("{tensor}");
    ///    // [[4, 4, 4], [4, 5, 6], [7, 8, 9]]
    /// }
    /// ```
    pub fn clamp_min<E: ElementConversion>(self, min: E) -> Self {
        Self::new(K::clamp_min(self.primitive, min.elem()))
    }

    /// Clamp element wise over a maximum value.
    ///
    /// # Arguments
    ///
    /// * `tensor` - The tensor to clamp.
    /// * `max` - The maximum value.
    ///
    /// # Returns
    ///
    /// A new tensor with the values clamped over the given max value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///    let device = Default::default();
    ///    let tensor = Tensor::<B, 2, Int>::from_ints(
    ///    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    ///    &device);
    ///    let tensor = tensor.clamp_max(5);
    ///    println!("{tensor}");
    ///    // [[1, 2, 3], [4, 5, 5], [5, 5, 5]]
    /// }
    /// ```
    pub fn clamp_max<E: ElementConversion>(self, max: E) -> Self {
        Self::new(K::clamp_max(self.primitive, max.elem()))
    }

    /// Apply element wise absolute value operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///   let device = Default::default();
    ///   let tensor = Tensor::<B, 2, Int>::from_ints([[1, -2, 3], [4, -5, 6], [7, -8, 9]], &device);
    ///   let tensor = tensor.abs();
    ///   println!("{tensor}");
    ///   // [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    /// }
    /// ```
    pub fn abs(self) -> Self {
        Self::new(K::abs(self.primitive))
    }

    /// Returns the upper triangular part of a matrix (2-D tensor) or batch of matrices input,
    /// the other elements of the result tensor out are set to 0.
    ///
    /// See also [`triu_mask`](Tensor::triu_mask).
    ///
    /// # Arguments
    ///
    /// * `diagonal` - The offset from the diagonal, where 0 means the diagonal, and positive values shift
    ///   towards the upper triangle.
    ///
    /// # Example
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///    let device = Default::default();
    ///    let tensor = Tensor::<B, 2, Int>::from_ints(
    ///        [
    ///          [1, 2, 3],
    ///          [4, 5, 6],
    ///          [7, 8, 9]
    ///        ],
    ///        &device
    ///    );
    ///    let tensor = tensor.triu(1);
    ///    println!("{tensor}");
    ///    // [
    ///    //   [0, 2, 3],
    ///    //   [0, 0, 6],
    ///    //   [0, 0, 0]
    ///    // ]
    /// }
    /// ```
    pub fn triu(self, diagonal: i64) -> Self {
        check!(TensorCheck::tri::<{ D }>());

        // last two dimensions
        let shape = &self.shape().dims[D - 2..].to_owned();

        let mask = Tensor::<B, 2, Bool>::triu_mask(shape, diagonal, &self.device()).unsqueeze();
        self.mask_fill(mask, 0)
    }

    /// Returns the lower triangular part of a matrix (2-D tensor) or batch of matrices input,
    /// the other elements of the result tensor out are set to 0.
    ///
    /// See also [`tril_mask`](Tensor::tril_mask).
    ///
    /// # Arguments
    ///
    /// * `diagonal` - The offset from the diagonal, where 0 means the diagonal, and positive values shift
    ///   towards the upper triangle.
    ///
    /// # Example
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Int, Tensor};
    ///
    /// fn example<B: Backend>() {
    ///    let device = Default::default();
    ///    let tensor = Tensor::<B, 2, Int>::from_ints(
    ///        [
    ///          [1, 2, 3],
    ///          [4, 5, 6],
    ///          [7, 8, 9]
    ///        ],
    ///        &device
    ///    );
    ///
    ///    let tensor = tensor.tril(-1);
    ///    println!("{tensor}");
    ///    // [
    ///    //   [0, 0, 0],
    ///    //   [4, 0, 0],
    ///    //   [7, 8, 0]
    ///    // ]
    /// }
    /// ```
    pub fn tril(self, diagonal: i64) -> Self {
        check!(TensorCheck::tri::<{ D }>());

        // last two dimensions
        let shape = &self.shape().dims[D - 2..].to_owned();
        let mask = Tensor::<B, 2, Bool>::tril_mask(shape, diagonal, &self.device()).unsqueeze();

        self.mask_fill(mask, 0)
    }

    /// Applies element wise power operation with a float Tensor
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to apply the power operation with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[2.0, 3.0, 4.0], [1.0, 2.0, 3.0]], &device);
    ///    let tensor = tensor1.powf(tensor2);
    ///    println!("{tensor}");
    ///    // [[1.0, 8.0, 81.0], [5.0, 81.0, 216.0]]
    /// }
    /// ```
    pub fn powf(self, other: Self) -> Self {
        Self::new(K::powf(self.primitive, other.primitive))
    }

    /// Applies element wise power operation with a float scalar
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to apply the power operation with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.powf_scalar(2.0);
    ///    println!("{tensor}");
    ///    // [[1.0, 4.0, 9.0], [25.0, 81.0, 36.0]]
    /// }
    /// ```
    pub fn powf_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::powf_scalar::<E>(self.primitive, other))
    }

    /// Applies element wise power operation with a integer Tensor
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to apply the power operation with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Int};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2, Int>::from_ints([[1, -2, 3], [5, 9, 6]], &device);
    ///    let tensor2 = Tensor::<B, 2, Int>::from_ints([[2, 3, 4], [1, 2, 3]], &device);
    ///    let tensor = tensor1.powi(tensor2);
    ///    println!("{tensor}");
    ///    // [[1, -8, 81], [5, 81, 216]]
    /// }
    /// ```
    pub fn powi(self, other: Self) -> Self {
        Self::new(K::powi(self.primitive, other.primitive))
    }

    /// Applies element wise power operation with a integer scalar
    ///
    /// # Arguments
    ///
    /// * `other` - The scalar to apply the power operation with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Int};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2, Int>::from_ints([[1, -2, 3], [5, 9, 6]], &device);
    ///    let tensor = tensor.powi_scalar(2);
    ///    println!("{tensor}");
    ///
    ///    // [[1, 4, 9], [25, 81, 36]]
    ///    let tensor = Tensor::<B, 2>::from_data([[1.5, -2., 3.], [5., 9., 6.]], &device);
    ///    let tensor = tensor.powi_scalar(2);
    ///    println!("{tensor}");
    ///    // [[2.25, 4., 9.], [25., 81., 36.]]
    /// }
    /// ```
    pub fn powi_scalar<E: ElementConversion>(self, other: E) -> Self {
        Self::new(K::powi_scalar::<E>(self.primitive, other))
    }

    /// Checks element wise if the tensor is close to another tensor.
    ///
    /// The tolerance is defined by the following equation:
    ///
    /// ```text
    /// abs(a - b) <= (atol + rtol * abs(b))
    ///
    /// where `a` is the first tensor, `b` is the second tensor, `rtol` is the relative tolerance,
    /// and `atol` is the absolute tolerance.
    /// ```
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to compare with.
    /// * `rtol` - Optional relative tolerance. Default is 1e-5; see `DEFAULT_RTOL`.
    /// * `atol` - Optional absolute tolerance. Default is 1e-8; see `DEFAULT_ATOL`.
    ///
    /// # Returns
    ///
    /// A boolean tensor with the same shape as the input tensors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor1.is_close(tensor2, None, None);
    ///    println!("{tensor}");
    ///    // [[true, true, true], [true, true, true]]
    /// }
    /// ```
    pub fn is_close(self, other: Self, rtol: Option<f64>, atol: Option<f64>) -> Tensor<B, D, Bool> {
        let rtol = rtol.unwrap_or(DEFAULT_RTOL);
        let atol = atol.unwrap_or(DEFAULT_ATOL);

        Tensor::new(K::lower_equal(
            K::abs(K::sub(self.primitive, other.primitive.clone())),
            K::add_scalar(K::mul_scalar(K::abs(other.primitive), rtol), atol),
        ))
    }

    /// Checks if all elements are close to another tensor.
    ///
    /// The tolerance is defined by the following equation:
    ///
    /// ```text
    ///
    /// abs(a - b) <= (atol + rtol * abs(b))
    ///
    /// where `a` is the first tensor, `b` is the second tensor, `rtol` is the relative tolerance,
    /// and `atol` is the absolute tolerance.
    ///
    /// ```
    ///
    /// # Arguments
    ///
    /// * `other` - The tensor to compare with.
    /// * `rtol` - Optional relative tolerance. Default is 1e-5; see `DEFAULT_RTOL`.
    /// * `atol` - Optional absolute tolerance. Default is 1e-8; see `DEFAULT_ATOL`.
    ///
    /// # Returns
    ///
    /// A boolean scalar.
    ///
    /// # Remarks
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor1 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor2 = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let result = tensor1.all_close(tensor2, None, None);
    ///    println!("{}", result);
    ///    // true
    /// }
    /// ```
    pub fn all_close(self, other: Self, rtol: Option<f64>, atol: Option<f64>) -> bool {
        self.is_close(other, rtol, atol)
            .all()
            .into_scalar()
            .to_bool()
    }

    /// Converts the tensor to a boolean tensor by checking if the elements are non-zero.
    ///
    /// # Returns
    ///
    /// A boolean tensor with the same shape as the input tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [0.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.bool();
    ///   println!("{tensor}");
    ///   // [
    ///   //   [true, true, true],
    ///   //   [false, true, true]
    ///   // ]
    /// }
    pub fn bool(self) -> Tensor<B, D, Bool> {
        Tensor::new(K::not_equal_elem(self.primitive, 0.elem()))
    }

    /// Create a random tensor of the given shape on the given device where each element is
    /// sampled from the given distribution.
    ///
    /// See also [`random_like`](Tensor::random_like).
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape of the tensor.
    /// * `distribution` - The distribution to sample from.
    /// * `device` - The device to create the tensor on.
    ///
    /// # Returns
    ///
    /// A new tensor with the given shape and elements sampled from the given distribution.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape, Distribution};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let distribution = Distribution::Uniform(0.0, 1.0); // Any random value between 0.0 and 1.0
    ///   let tensor = Tensor::<B, 2>::random(Shape::new([2, 3]), distribution, &device);
    ///   println!("{tensor}");
    ///   // [
    ///   //   [0.08347523, 0.70498955, 0.60332155],
    ///   //   [0.08173251, 0.18028641, 0.97942924]
    ///   // ]
    /// }
    /// ```
    pub fn random<S: Into<Shape>>(
        shape: S,
        distribution: Distribution,
        device: &B::Device,
    ) -> Self {
        Self::new(K::random(shape.into(), distribution, device))
    }

    /// Sort the elements by value in ascending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Returns
    ///
    /// A new tensor with the elements sorted in ascending order along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///   let tensor = tensor.sort(0);
    ///   println!("{tensor}");
    ///   // [[5.0, -2.0, 3.0], [12.0, 3.0, 6.0]]
    ///   let tensor = tensor.sort(1);
    ///   println!("{tensor}");
    ///   // [[-2.0, 3.0, 12.0], [3.0, 5.0, 6.0]]
    /// }
    /// ```
    pub fn sort(self, dim: usize) -> Tensor<B, D, K> {
        check!(TensorCheck::sort_dim::<D>("Sort", dim));
        Tensor::new(K::sort(self.primitive, dim, /*descending*/ false))
    }

    /// Sort the elements by value in descending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Returns
    ///
    /// A new tensor with the elements sorted in descending order along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let tensor = tensor.sort_descending(0);
    ///    println!("{tensor}");
    ///    // [[12.0, 3.0, 6.0], [5.0, -2.0, 3.0]]
    ///    let tensor = tensor.sort_descending(1);
    ///    println!("{tensor}");
    ///    // [[12.0, 3.0, -2.0], [6.0, 5.0, 3.0]]
    /// }
    /// ```
    pub fn sort_descending(self, dim: usize) -> Tensor<B, D, K> {
        check!(TensorCheck::sort_dim::<D>("Sort", dim));
        Tensor::new(K::sort(self.primitive, dim, /*descending*/ true))
    }

    /// Sort the elements by value in ascending order along a given dimension.
    /// Also returns the indices.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Returns
    ///
    /// A tuple containing the sorted tensor and the indices tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///   let (tensor, indices) = tensor.sort_with_indices(0);
    ///   println!("{tensor}");
    ///   // [[5.0, -2.0, 3.0], [12.0, 3.0, 6.0]]
    ///   println!("{}", indices);
    ///   // [[1, 0, 0], [0, 1, 1]]
    /// }
    /// ```
    pub fn sort_with_indices(self, dim: usize) -> (Tensor<B, D, K>, Tensor<B, D, Int>) {
        check!(TensorCheck::sort_dim::<D>("Sort_with_indices", dim));
        let (values, indices) =
            K::sort_with_indices(self.primitive, dim, /*descending*/ false);
        (Tensor::new(values), Tensor::new(indices))
    }

    /// Sort the elements by value in descending order along a given dimension.
    /// Also returns the indices.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let (tensor, indices) = tensor.sort_descending_with_indices(0);
    ///    println!("{tensor}");
    ///    // [[12.0, 3.0, 6.0], [5.0, -2.0, 3.0]]
    ///    println!("{}", indices);
    ///    // [[0, 1, 1], [1, 0, 0]]
    /// }
    /// ```
    pub fn sort_descending_with_indices(self, dim: usize) -> (Tensor<B, D, K>, Tensor<B, D, Int>) {
        check!(TensorCheck::sort_dim::<D>("Sort_with_indices", dim));
        let (values, indices) = K::sort_with_indices(self.primitive, dim, /*descending*/ true);
        (Tensor::new(values), Tensor::new(indices))
    }

    /// Returns the indices that sort the elements by value in ascending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let tensor = tensor.argsort(0);
    ///    println!("{tensor}");
    ///    // [[1, 0, 0], [0, 1, 1]]
    /// }
    /// ```
    pub fn argsort(self, dim: usize) -> Tensor<B, D, Int> {
        check!(TensorCheck::sort_dim::<D>("Argsort", dim));
        Tensor::new(K::argsort(self.primitive, dim, /*descending*/ false))
    }

    /// Returns the indices that sort the elements by value in descending order along a given dimension.
    ///
    /// This sort is unstable (i.e., may reorder equal elements).
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension to sort along.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let tensor = tensor.argsort_descending(0);
    ///    println!("{tensor}");
    ///    // [[0, 1, 1], [1, 0, 0]]
    ///    let tensor = tensor.argsort_descending(1);
    ///    println!("{tensor}");
    ///    // [[0, 2, 1], [2, 0, 1]]
    /// }
    /// ```
    pub fn argsort_descending(self, dim: usize) -> Tensor<B, D, Int> {
        check!(TensorCheck::sort_dim::<D>("Argsort", dim));
        Tensor::new(K::argsort(self.primitive, dim, /*descending*/ true))
    }

    /// Returns the `k` largest elements of the given input tensor along a given dimension.
    ///
    /// # Arguments
    ///
    /// * `k` - The number of elements to return.
    ///
    /// # Returns
    ///
    /// A new tensor with the `k` largest elements along the given dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///   let tensor = tensor.topk(2, 0);
    ///   println!("{tensor}");
    ///   // [[12.0, 3.0, 6.0], [5.0, -2.0, 3.0]]
    ///   let tensor = tensor.topk(1, 1);
    ///   println!("{tensor}");
    ///   // [[12.0], [6.0]]
    /// }
    /// ```
    pub fn topk(self, k: usize, dim: usize) -> Tensor<B, D, K> {
        let k_indices = Tensor::arange(0..k as i64, &self.device());
        self.sort_descending(dim).select(dim, k_indices)
    }

    /// Returns the `k` largest elements of the given input tensor along a given dimension.
    /// Also returns the indices.
    ///
    /// # Arguments
    ///
    /// * `k` - The number of elements to return.
    /// * `dim` - The dimension to sort along.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let (tensor, indices) = tensor.topk_with_indices(2, 0);
    ///    println!("{tensor}");
    ///    // [[12.0, 3.0, 6.0], [5.0, -2.0, 3.0]]
    ///    println!("{}", indices);
    ///    // [[0, 1, 1], [1, 0, 0]]
    ///    let (tensor, indices) = tensor.topk_with_indices(1, 1);
    ///    println!("{tensor}");
    ///    // [[12.0], [6.0]]
    ///    println!("{indices}");
    ///    // [[0], [2]]
    /// }
    /// ```
    pub fn topk_with_indices(self, k: usize, dim: usize) -> (Tensor<B, D, K>, Tensor<B, D, Int>) {
        let k_indices = Tensor::arange(0..k as i64, &self.device());
        let (values, indices) = self.sort_descending_with_indices(dim);
        (
            values.select(dim, k_indices.clone()),
            indices.select(dim, k_indices),
        )
    }

    /// Pad the tensor of rank two or higher with the given value on the last two dimensions.
    ///
    /// # Arguments
    ///
    /// * `padding` - A tuple of four integers representing the padding on the left, right, top, and bottom.
    /// * `value` - The value to pad the tensor with.
    ///
    /// # Returns
    ///
    /// A new tensor with the given padding.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Shape};
    ///
    /// fn example<B: Backend<FloatElem: From<f32>>>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[12.0, -2.0, 3.0], [5.0, 3.0, 6.0]], &device);
    ///    let tensor = tensor.pad((1, 1, 1, 1), 0.0);
    ///    println!("{tensor}");
    ///    // [
    ///    //   [0.0, 0.0, 0.0, 0.0, 0.0],
    ///    //   [0.0, 12.0, -2.0, 3.0, 0.0],
    ///    //   [0.0, 5.0, 3.0, 6.0, 0.0],
    ///    //   [0.0, 0.0, 0.0, 0.0, 0.0]
    ///    // ]
    /// }
    /// ```
    pub fn pad<E: ElementConversion>(
        self,
        padding: (usize, usize, usize, usize),
        value: E,
    ) -> Tensor<B, D, K> {
        let (left, right, top, bottom) = padding;

        let mut padded_dims: [usize; D] = self.dims();

        // Update the last two dimensions with padding
        padded_dims[D - 2] += top + bottom;
        padded_dims[D - 1] += left + right;

        // Create the ranges for the padded tensor
        let ranges: [core::ops::Range<usize>; D] = padded_dims
            .iter()
            .enumerate()
            .map(|(i, &dim)| {
                if i == D - 2 {
                    top..dim - bottom
                } else if i == D - 1 {
                    left..dim - right
                } else {
                    0..dim
                }
            })
            .collect::<Vec<core::ops::Range<usize>>>()
            .try_into()
            .unwrap();

        // Create the padded tensor
        let padded_tensor = Tensor::full(padded_dims, value, &self.device());

        // Assign the original tensor data to the appropriate slice of the padded tensor
        padded_tensor.slice_assign(ranges, self)
    }
    /// Create a one hot tensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::Tensor;
    ///
    /// fn example<B: Backend>(){
    ///     let device = Default::default();
    ///     let indices: Tensor<B, 1> = Tensor::from_floats([0.0, 1.0, 2.0, 3.0], &device);
    ///     let one_hot: Tensor<B, 2> = indices.one_hot(4);
    ///     println!("{}", one_hot.to_data());
    ///     // [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
    /// }
    /// ```
    pub fn one_hot<const D2: usize>(self, num_classes: usize) -> Tensor<B, D2, K> {
        check!(TensorCheck::one_hot_tensor(self.clone(), num_classes));
        self.one_hot_fill(num_classes, 1.0, 0.0, -1)
    }

    /// Create a one-hot encoded tensor with configurable `num_classes`, `on_value`, `off_value`, and `axis` including high-ranked tensors.
    ///
    /// # Arguments
    ///
    /// * `num_classes`: The number of classes for the one-hot encoding, which defines the size of the one-hot dimension.
    /// * `on_value`: The value to assign for active positions (corresponding to indices).
    /// * `off_value`: The value to assign for inactive positions.
    /// * `axis`: The axis along which the one-hot dimension is added. Supports negative indexing.
    ///
    /// # Returns
    ///
    /// A tensor with one additional dimension for the one-hot encoding, where active positions are filled with `on_value` and others with `off_value`.
    ///
    /// # Example
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Float};
    /// fn example<B: Backend<FloatElem: From<f32>>>() {
    ///     let device = B::Device::default();
    ///     let indices: Tensor<B, 2, Float> = Tensor::from_floats([[0., 2.], [1., -1.]], &device);
    ///     // One-hot encoding
    ///     let tensor:Tensor<B, 3, Float> = indices.one_hot_fill(3, 5.0.into(), 0.0.into(), -1);
    ///     println!("{tensor}");
    ///     // [[[5.0, 0.0, 0.0],
    ///     // [0.0, 0.0, 5.0]],
    ///     // [[0.0, 5.0, 0.0],
    ///     // [0.0, 0.0, 5.0]]]
    /// }
    /// ```
    pub fn one_hot_fill<const D2: usize>(
        self,
        num_classes: usize,
        on_value: f32,
        off_value: f32,
        axis: i64,
    ) -> Tensor<B, D2, K> {
        check!(TensorCheck::one_hot_tensor_rank::<D, D2>());
        // Initialize shape from the current tensor dimensions and prepare for modification
        let mut shape = self.shape().dims::<D>().to_vec();
        let device = self.device();
        let rank = self.dims().len();

        // Adjust negative axis to a positive index
        let axis = if axis < 0 {
            axis + rank as i64 + 1
        } else {
            axis
        };

        // Ensure axis is within valid range
        if axis < 0 || axis > rank as i64 {
            panic!("Axis out of range. Accepted range is [-r-1, r] where r = rank(indices).");
        }
        // Convert the input tensor to integer indices
        let indices: Tensor<B, D, Int> =
            Tensor::from_data(self.to_data().convert::<i64>(), &device);
        // Insert the new dimension for the one-hot representation
        shape.insert(axis as usize, num_classes);
        // Adjust indices to valid range and handle invalid indices
        let adjusted_indices = indices
            .clone()
            .mask_fill(self.clone().lower_elem(0), num_classes as i64) // Handle negative indices
            .add(indices.clone().mask_fill(self.clone().greater_elem(0), 0)); // Handle positive indices
        // Unsqueeze the indices tensor along the specified axis
        let indices_unsqueezed: Tensor<B, D2, Int> = adjusted_indices.unsqueeze_dim(axis as usize);

        // Initialize the output tensor with the off_value
        let output = Tensor::full(shape.clone(), off_value, &device);

        // Prepare scatter tensor for on_value and off_value adjustments
        let scatter_on_values = Tensor::full(indices_unsqueezed.shape(), on_value, &device)
            - Tensor::full(indices_unsqueezed.shape(), off_value, &self.device());

        // Scatter on_value at the appropriate indices to create the one-hot representation
        output.scatter(axis as usize, indices_unsqueezed, scatter_on_values)
    }

    /// Returns a new tensor with boolean elements indicating whether each element of the input is NaN.
    ///
    /// # Returns
    ///
    /// A boolean tensor where `true` indicates NaN and `false` indicates a non-NaN value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Bool, Shape};
    ///
    /// fn example<B: Backend>() {
    ///    let device = B::Device::default();
    ///    let tensor = Tensor::<B, 2>::from_data([[1.0, f64::NAN, 3.0], [5.0, 9.0, 6.0]], &device);
    ///    let tensor = tensor.is_nan();
    ///    println!("{tensor}");
    ///    // [[false, true, false], [false, false, false]]
    /// }
    /// ```
    pub fn is_nan(&self) -> Tensor<B, D, Bool> {
        // Check if the input tensor is NaN by comparing it to itself
        // NaN is the only value that is not equal to itself
        Tensor::new(K::not_equal(self.primitive.clone(), self.primitive.clone()))
    }

    /// Checks if the tensor contains any NaN values.
    ///
    /// # Returns
    ///
    /// A boolean tensor with a single element indicating whether the tensor contains any NaN values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use burn_tensor::backend::Backend;
    /// use burn_tensor::{Tensor, Bool, Shape};
    ///
    /// fn example<B: Backend>() {
    ///   let device = B::Device::default();
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [f64::NAN, 9.0, 6.0]], &device);
    ///   let tensor = tensor.contains_nan();
    ///   println!("{tensor}");
    ///   // [true]
    ///   let tensor = Tensor::<B, 2>::from_data([[1.0, -2.0, 3.0], [5.0, 9.0, 6.0]], &device);
    ///   let tensor = tensor.contains_nan();
    ///   println!("{tensor}");
    ///   // [false]
    /// }
    /// ```
    pub fn contains_nan(&self) -> Tensor<B, 1, Bool> {
        // Summing the tensor will result in NaN if the tensor contains any NaN values
        // This is faster than checking each element individually
        // because it rolls up the NaN values into a single value
        let sum = K::sum(self.primitive.clone());

        // Check if the sum is NaN by comparing it to itself
        Tensor::new(K::not_equal(sum.clone(), sum))
    }
}

impl<B, K> Tensor<B, 2, K>
where
    B: Backend,
    K: Numeric<B>,
    K::Elem: Element,
{
    /// Creates a new 2D tensor with ones on the diagonal and zeros elsewhere.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the square matrix.
    pub fn eye(size: usize, device: &B::Device) -> Self {
        let indices = Tensor::<B, 1, Int>::arange(0..size as i64, device).unsqueeze::<2>();
        let ones = K::ones([1, size].into(), device);
        let zeros = K::zeros([size, size].into(), device);

        Self::new(K::scatter(0, zeros, indices.primitive, ones))
    }
}

// Tensor + tensor
impl<B: Backend, const D: usize, K: Numeric<B>> core::ops::Add<Self> for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn add(self, rhs: Tensor<B, D, K>) -> Self::Output {
        Self::add(self, rhs)
    }
}

// Tensor + scalar
impl<E: ElementConversion, const D: usize, B: Backend, K: Numeric<B>> core::ops::Add<E>
    for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn add(self, other: E) -> Self::Output {
        Tensor::add_scalar(self, other)
    }
}

// Scalar + tensor
macro_rules! impl_tensor_scalar_add {
    ($($t:ty),*) => {
        $(
            impl<const D: usize, B: Backend, K: Numeric<B>> core::ops::Add<Tensor<B, D, K>> for $t
            where
                K::Elem: Element,
            {
                type Output = Tensor<B, D, K>;

                fn add(self, tensor: Tensor<B, D, K>) -> Self::Output {
                    Tensor::add_scalar(tensor, self)
                }
            }
        )*
    }
}
impl_tensor_scalar_add!(f32, f64, i32, i64, u32, u64);

// Tensor - tensor
impl<B: Backend, const D: usize, K: Numeric<B>> core::ops::Sub<Self> for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn sub(self, rhs: Tensor<B, D, K>) -> Self::Output {
        Tensor::sub(self, rhs)
    }
}

// Tensor - scalar
impl<E: ElementConversion, const D: usize, B: Backend, K: Numeric<B>> core::ops::Sub<E>
    for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn sub(self, other: E) -> Self::Output {
        Tensor::sub_scalar(self, other)
    }
}

// Scalar - tensor
macro_rules! impl_tensor_scalar_sub {
    ($($t:ty),*) => {
        $(
            impl<const D: usize, B: Backend, K: Numeric<B>> core::ops::Sub<Tensor<B, D, K>> for $t
            where
                K::Elem: Element,
            {
                type Output = Tensor<B, D, K>;

                fn sub(self, tensor: Tensor<B, D, K>) -> Self::Output {
                    Tensor::add_scalar(Tensor::neg(tensor), self)
                }
            }
        )*
    }
}
impl_tensor_scalar_sub!(f32, f64, i32, i64, u32, u64);

// Tensor / tensor
impl<B: Backend, const D: usize, K: Numeric<B>> core::ops::Div<Self> for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Tensor::div(self, rhs)
    }
}

// Tensor / scalar
impl<E: ElementConversion, const D: usize, B: Backend, K: Numeric<B>> core::ops::Div<E>
    for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn div(self, other: E) -> Self::Output {
        Tensor::div_scalar(self, other)
    }
}

// Scalar / tensor (float only)
macro_rules! impl_tensor_scalar_div {
    ($($t:ty),*) => {
        $(
            impl<const D: usize, B: Backend> core::ops::Div<Tensor<B, D>> for $t
            {
                type Output = Tensor<B, D>;

                fn div(self, tensor: Tensor<B, D>) -> Self::Output {
                    tensor.recip().mul_scalar(self)
                }
            }
        )*
    }
}

impl_tensor_scalar_div!(f32, f64);

// Tensor % tensor.
impl<const D: usize, B: Backend, K: Numeric<B>> core::ops::Rem<Self> for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Tensor::remainder(self, rhs)
    }
}

// Tensor % scalar.
impl<E: ElementConversion, const D: usize, B: Backend, K: Numeric<B>> core::ops::Rem<E>
    for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn rem(self, other: E) -> Self::Output {
        Tensor::remainder_scalar(self, other)
    }
}

// Tensor * tensor.
impl<B: Backend, const D: usize, K: Numeric<B>> core::ops::Mul<Self> for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Tensor::mul(self, rhs)
    }
}

// Tensor * scalar.
impl<E: ElementConversion, const D: usize, B: Backend, K: Numeric<B>> core::ops::Mul<E>
    for Tensor<B, D, K>
where
    K::Elem: Element,
{
    type Output = Self;

    fn mul(self, other: E) -> Self::Output {
        Tensor::mul_scalar(self, other)
    }
}

macro_rules! impl_tensor_scalar_mul {
    ($($t:ty),*) => {
        $(
            impl<const D: usize, B: Backend, K: Numeric<B>> core::ops::Mul<Tensor<B, D, K>> for $t
            where
                K::Elem: Element,
            {
                type Output = Tensor<B, D, K>;

                fn mul(self, other: Tensor<B, D, K>) -> Self::Output {
                    Tensor::mul_scalar(other, self)
                }
            }
        )*
    }
}

impl_tensor_scalar_mul!(f32, f64, i32, i64, u32, u64);

impl<B, const D: usize, K> core::ops::Neg for Tensor<B, D, K>
where
    B: Backend,
    K: Numeric<B>,
    K::Elem: Element,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tensor::neg(self)
    }
}
