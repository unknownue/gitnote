
import torch
import torch.nn.functional as F
import torchvision

from collections import OrderedDict

from network import Network
from run_manager import RunManager
from run_builder import RunBuilder


torch.set_printoptions(linewidth=120)  # Display options for output
torch.set_grad_enabled(True)  # Already on by default

data_transform = torchvision.transforms.Compose([
    torchvision.transforms.ToTensor()
])
train_set = torchvision.datasets.FashionMNIST(root='./data/', train=True, download=True, transform=data_transform)

params = OrderedDict(
    lr = [0.01],
    batch_size = [1000, 2000],
    num_workers = [2],
)

manager = RunManager()

for run in RunBuilder.get_runs(params):

    network = Network()
    train_loader = torch.utils.data.DataLoader(train_set, batch_size=run.batch_size, num_workers=run.num_workers, shuffle=True)
    optimizer = torch.optim.Adam(network.parameters(), lr=run.lr)

    manager.begin_run(run, network, train_loader)

    for epoch in range(5):
        manager.begin_epoch()

        for images, labels in train_loader: # Get Batch

            preds = network(images) # Pass Batch
            loss  = F.cross_entropy(preds, labels) # Calculate Loss
            
            optimizer.zero_grad()
            loss.backward() # Calculate Gradients
            optimizer.step() # Update Weights

            manager.track_loss(loss)
            manager.track_num_correct(preds, labels)

        manager.end_epoch()
    manager.end_run()

manager.save('results')

