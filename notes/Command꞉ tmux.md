---
tags: [Command Line]
title: 'Command: tmux'
created: '2020-01-19T01:47:17.532Z'
modified: '2020-02-22T15:53:47.782Z'
---

# Command: tmux

Personal keymapping for tmux

Keymapping

prefix
```
Ctrl + `
```

Create/Kill/List a new session
```shell
tmux new -s session_name
tmux kill-session -t session_name
tmux ls
```

attach to exist session
```shell
tmux a  # attach first session
tmux a -t session_name
```

Maximize a pane
```
prefix + z
```

Merge a pane to window
```
prefix + :
join-pane -s window01  # Merge the first panel of window01 to current window
join-pane -s window01.2 # Merge the second panel of window01 to current window
```

Close a pane
```
prefix + x # Then confirm with y
```

See also http://louiszhai.github.io/2017/09/30/tmux/

