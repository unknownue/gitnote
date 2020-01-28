---
tags: [Python]
title: PyTorch Note
created: '2020-01-28T08:17:20.703Z'
modified: '2020-01-28T08:23:14.793Z'
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

