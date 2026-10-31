| Layer | Responsibility | Examples |
| --- | --- | --- |
| Storage and kernels | Own numeric data and perform calculations | Elementwise addition, matrix multiplication |
| Tensor | Track shape, shared data, and autodiff metadata | `Tensor`, `reshape`, `sum` |
| Autograd | Record operations and propagate gradients | `backward`, gradient accumulation |
| Neural networks | Compose trainable operations | `Linear`, `ReLU`, loss functions |
| Optimizers | Update trainable tensors | SGD |