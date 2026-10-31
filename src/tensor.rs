pub struct Tensor (Rc<TensorInner>);
// Obs: this is a single threaded implementation 
struct TensorInner{
    id : TensorId,
    data: RefCell<Vec<f32>>,
    shape: Vec<usize>,

    requires_grad: bool,
    grad: RefCell<Option<Tensor>>,

    grad_fn: Option<BackwardOp0>,
}