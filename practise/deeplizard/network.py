
import torch
import torch.nn.functional as F

class Network(torch.nn.Module):

    def __init__(self):
        super().__init__()

        self.conv1 = torch.nn.Conv2d(in_channels=1, out_channels=6 , kernel_size=5)
        self.conv2 = torch.nn.Conv2d(in_channels=6, out_channels=12, kernel_size=5)

        self.fc1 = torch.nn.Linear(in_features=12 * 4 * 4, out_features=120)
        self.fc2 = torch.nn.Linear(in_features=120, out_features=60)
        self.out = torch.nn.Linear(in_features=60, out_features=10)

    def forward(self, t: torch.Tensor):

        t = F.relu(self.conv1(t))
        t = F.max_pool2d(t, kernel_size=2, stride=2)

        t = F.relu(self.conv2(t))
        t = F.max_pool2d(t, kernel_size=2, stride=2)
        
        t = t.flatten(start_dim=1)
        t = F.relu(self.fc1(t))

        t = F.relu(self.fc2(t))
        t = self.out(t)

        return t

