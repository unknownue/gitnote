---
tags: [Python]
title: PyTorch Note
created: '2020-01-28T08:17:20.703Z'
modified: '2020-01-30T13:45:42.484Z'
---

# PyTorch Note

```Python
import torch
```

### tensor constructor
```Python
# Construct tensor by copy in default data type(torch.float32)
torch.Tensor(data)
```

```Python
# Construct tensor by copy(data type is inferred by input)
torch.tensor(data)
```

```Python
# Construct tensor by reference
torch.as_tensor()
torch.from_numpy()
```



## CNN Output Size Formula

Let's have a look at the formula for computing the output size of the tensor after performing convolutional and pooling operations.
CNN Output Size Formula (Square)

- Suppose we have an $(n \times n)$ input.
- Suppose we have an $(f \times f)$ filter.
- Suppose we have a padding of  $p$ and a stride of $s$.

The output size \(O\) is given by this formula:

$$
O = \frac{n - f + 2p}{s} + 1
$$