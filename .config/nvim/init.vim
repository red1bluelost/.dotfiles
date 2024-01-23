call plug#begin(stdpath('data') . '/plugged')

Plug 'nvim-lualine/lualine.nvim'
Plug 'nvim-tree/nvim-web-devicons'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'nvim-treesitter/nvim-treesitter', {'do': ':TSUpdate'}
Plug 'akinsho/bufferline.nvim'

Plug 'norcalli/nvim-colorizer.lua'

Plug 'rhysd/vim-clang-format'

Plug 'rust-lang/rust.vim'

Plug 'preservim/tagbar'
Plug 'universal-ctags/ctags'

Plug 'tpope/vim-fugitive'
Plug 'sindrets/diffview.nvim'

Plug 'mattn/webapi-vim'

Plug 'duane9/nvim-rg'

Plug 'numToStr/Comment.nvim'
Plug 'sbdchd/neoformat'

Plug 'red1bluelost/darcula.nvim'

call plug#end()

let mapleader = " "

" better tab completion
set wildmode=longest,list,full

"coc extensions to use
let g:coc_global_extensions = [
      \'coc-clangd',
      \'coc-highlight',
      \'coc-git',
      \'coc-gitignore',
      \'coc-json',
      \'coc-pairs',
      \'coc-rust-analyzer',
      \'coc-sumneko-lua',
      \'coc-vimlsp',
      \]

" rust.vim options
let g:rustfmt_autosave = 1

" coc key remapping
nnoremap <Leader>d <Plug>(coc-definition)
nnoremap <Leader>p <Plug>(coc-declaration)
nnoremap <Leader>i <Plug>(coc-implementation)
nnoremap <Leader>t <Plug>(coc-type-definition)
nnoremap <Leader>r <Plug>(coc-references)
nnoremap <Leader>u <Plug>(coc-references-used)
" coc highlight variables under cursor
autocmd CursorHold * silent call CocActionAsync('highlight')

" Color scheme options
set background=dark
set termguicolors
colorscheme darcula
hi! link CocErrorSign ErrorSign
hi! link CocWarningSign WarningSign
hi! link CocInfoSign InfoSign
hi! link CocHintSign HintSign
hi! link CocErrorFloat Pmenu
hi! link CocWarningFloat Pmenu
hi! link CocInfoFloat Pmenu
hi! link CocHintFloat Pmenu
hi! link CocHighlightText IdentifierUnderCaret
hi! link CocHighlightRead IdentifierUnderCaret
hi! link CocHighlightWrite IdentifierUnderCaretWrite
hi! link CocErrorHighlight CodeError
hi! link CocWarningHighlight CodeWarning
hi! link CocInfoHighlight CodeInfo
hi! link CocHintHighlight CodeHint

" Whitespace at the end of a line. This little dance suppresses
" whitespace that has just been typed.
au BufWinEnter * let w:m1=matchadd('WhitespaceEOL', '\s\+$', -1)
au InsertEnter * call matchdelete(w:m1)
au InsertEnter * let w:m2=matchadd('WhitespaceEOL', '\s\+\%#\@<!$', -1)
au InsertLeave * call matchdelete(w:m2)
au InsertLeave * let w:m1=matchadd('WhitespaceEOL', '\s\+$', -1)

lua require('config')
