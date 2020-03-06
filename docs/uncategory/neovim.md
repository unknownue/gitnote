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

## Substitute
在VIM中进行文本替换：
 
1.  替换当前行中的内容：    :s/from/to/    （s即substitude）
:s/from/to/     ：  将当前行中的第一个from，替换成to。如果当前行含有多个
		    from，则只会替换其中的第一个。
:s/from/to/g    ：  将当前行中的所有from都替换成to。
:s/from/to/gc   ：  将当前行中的所有from都替换成to，但是每一次替换之前都
		    会询问请求用户确认此操作。

注意：这里的from和to都可以是任何字符串，其中from还可以是正则表达式。

2.  替换某一行的内容：      :33s/from/to/g
:.s/from/to/g   ：  在当前行进行替换操作。
:33s/from/to/g  ：  在第33行进行替换操作。
:$s/from/to/g   ：  在最后一行进行替换操作。

3.  替换某些行的内容：      :10,20s/from/to/g
:10,20s/from/to/g   ：  对第10行到第20行的内容进行替换。
:1,$s/from/to/g     ：  对第一行到最后一行的内容进行替换（即全部文本）。
:1,.s/from/to/g     ：  对第一行到当前行的内容进行替换。
:.,$s/from/to/g     ：  对当前行到最后一行的内容进行替换。
:'a,'bs/from/to/g   ：  对标记a和b之间的行（含a和b所在的行）进行替换。
			其中a和b是之前用m命令所做的标记。

4.  替换所有行的内容：      :%s/from/to/g
:%s/from/to/g   ：  对所有行的内容进行替换。

