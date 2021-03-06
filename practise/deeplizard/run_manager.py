
import time
from collections import OrderedDict

import torch
import torchvision
from torch.utils.tensorboard import SummaryWriter
import pandas


# ----------------------------------------------------------------------------------
class RunManager:

    def __init__(self):

        self.epoch_count = 0
        self.epoch_loss  = 0.0
        self.epoch_num_correct = 0
        self.epoch_start_time = None

        self.run_params = None
        self.run_count = 0
        self.run_data = []
        self.run_start_time = None

        self.network = None
        self.loader  = None
        self.tensorboard = None

    def begin_run(self, run, network: torch.nn.Module, loader: torch.utils.data.DataLoader):
        
        self.run_start_time = time.time()

        self.run_params = run
        self.run_count += 1

        self.network = network
        self.loader  = loader
        self.tensorboard = SummaryWriter(comment=f'-{run}')

        images, labels = next(iter(self.loader))
        grid = torchvision.utils.make_grid(images)

        self.tensorboard.add_image('images', grid)
        self.tensorboard.add_graph(self.network, images)

    def end_run(self):
        self.tensorboard.close()
        self.epoch_count = 0

    def begin_epoch(self):
        self.epoch_start_time = time.time()
        
        self.epoch_count += 1
        self.epoch_loss = 0.0
        self.epoch_num_correct = 0

    def end_epoch(self):

        epoch_duration = time.time() - self.epoch_start_time
        run_duration   = time.time() - self.run_start_time

        loss = self.epoch_loss / len(self.loader.dataset)
        accuracy = self.epoch_num_correct / len(self.loader.dataset)

        self.tensorboard.add_scalar('Loss', loss, self.epoch_count)
        self.tensorboard.add_scalar('Accuracy', accuracy, self.epoch_count)

        for name, params in self.network.named_parameters():
            self.tensorboard.add_histogram(name, params, self.epoch_count)
            self.tensorboard.add_histogram(f'{name}.grad', params.grad, self.epoch_count)
        
        results = OrderedDict()
        results['run']            = self.run_count
        results['epoch']          = self.epoch_count
        results['loss']           = loss
        results['accuracy']       = accuracy
        results['epoch duration'] = epoch_duration
        results['run duration']   = run_duration

        for k, v in self.run_params._asdict().items():
            results[k] = v
        self.run_data.append(results)
        
        df = pandas.DataFrame.from_dict(self.run_data, orient='columns')

        # For jupyter notebook
        # clear_output(wait=True)
        # display(df)
        
    def track_loss(self, loss):
        self.epoch_loss += loss.item() * self.loader.batch_size

    def track_num_correct(self, preds, labels):
        self.epoch_num_correct += RunManager._get_num_correct(preds, labels)

    @torch.no_grad()
    def _get_num_correct(preds: torch.Tensor, labels: torch.Tensor):
        return preds.argmax(dim=1).eq(labels).sum().item()

    def save(self, filename):
        pandas.DataFrame.from_dict(self.run_data, orient='columns').to_csv(f'{filename}.csv')
        
        # with open(f'{filename}.json', 'w', encoding='utf-8') as f:
        #     json.dump(self.run_data, f, ensure_ascii=False, indent=4)
# ----------------------------------------------------------------------------------

