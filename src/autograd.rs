enum BackwardOp{
    Add{
        left:Tensor,
        right:Tensor,
    },
    Matmul{
        left:Tensor,
        right:Tensor,
        // obs : saving the left and right values is not necessary for the backward pass,
        // but it after ill change it so better memory usage
        left_save: Option<f32>,
        right_save: Option<f32>,
    },
    Relu{
        input:Tensor,
        positive_mask: Vec<bool>,
    },
    Sum{
        input:Tensor,
    },
}