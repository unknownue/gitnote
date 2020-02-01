
import torch
import torch.nn.functional as F
import torchvision

torch.set_printoptions(linewidth=120)  # Display options for output
torch.set_grad_enabled(True)  # Already on by default

from torch.utils.tensorboard import SummaryWriter

def get_num_correct(preds: torch.Tensor, labels: torch.Tensor):
    return preds.argmax(dim=1).eq(labels).sum().item()



