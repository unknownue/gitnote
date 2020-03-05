# Neovim Note

Save a file with admin
```vimscript
:w !sudo tee %<CR>
```

Print to html
```vimscript
:%TOhtml<CR>
```

Inscrease number
```vimscript
<C-a>
```

```vimscript
# Open file
:e file.h
# open a file in horizontal
:sp filename
# open a file in vertical
:vsp filename 
```

```vimscript
# Split window for the same file
:sp
# Close current window
:close
# Preserve current window, close other window
:only
```

```vimscript
" Next buffer in buffer list
:bn
" Previous buffer in buffer list
:bp
" Close current buffer
:bd
```

```vimscript
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
