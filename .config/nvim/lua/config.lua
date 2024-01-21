vim.opt.number = true
vim.opt.softtabstop = 2
vim.opt.shiftwidth = 2
vim.opt.expandtab = true
vim.opt.colorcolumn = "80"

require'nvim-treesitter.configs'.setup {
  -- A list of parser names, or "all"
  ensure_installed = {
    "c",
    "cmake",
    "cpp",
    "haskell",
    "json",
    "llvm",
    "lua",
    "rust",
    "toml",
    "yaml",
  },

  -- Install parsers synchronously (only applied to `ensure_installed`)
  sync_install = false,

  -- Automatically install missing parsers when entering buffer
  auto_install = true,

  highlight = {
    enable = true,
  },
  incremental_selection = {
    enable = true,
  },
  indent = {
    enable = true,
  },
}

-- set up the comment plugin
require('Comment').setup{}

-- set up bufferline plugin
require("bufferline").setup{}

require('lualine').setup{}

require('colorizer').setup()
