---
tags: [Vim]
title: Neovim Note
created: '2020-01-22T13:55:41.399Z'
modified: '2020-02-01T05:37:56.496Z'
---

# Neovim Note

```vim
# Open file
:e file.h
# open a file in horizontal
:sp filename
# open a file in vertical
:vsp filename 
```

```vim
# Split window for the same file
:sp
# Close current window
:close
# Preserve current window, close other window
:only
```

```vim
" Next buffer in buffer list
:bn
" Previous buffer in buffer list
:bp
" Close current buffer
:bd
```

```vim
# Jump between window
<C-w>j  -> move to bottom
<C-w>k  -> move to up
<C-w>h  -> move to left
<C-w>h  -> move to right
<C-w>t  -> move to top position
<C-w>b  -> move to bottom position
# Move window
<C-w>r  -> swap window from right/bottom direction
<C-w>R  -> swap window from left/up direction
<C-w>x  -> swap window in same column or line
```
