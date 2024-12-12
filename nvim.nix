require('rust-tools').setup({})
require'nvim-treesitter.configs'.setup {
    ensure_installed = { "rust" },
    highlight = { enable = true },
}