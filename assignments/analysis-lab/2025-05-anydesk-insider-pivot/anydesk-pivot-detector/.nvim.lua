-- Neovim Local Configuration for AnyDesk Pivot Detector
-- This requires a plugin like 'neoconf.nvim' or manual loading

local lspconfig = require('lspconfig')

-- Rust Analyzer Setup
lspconfig.rust_analyzer.setup({
  settings = {
    ["rust-analyzer"] = {
      checkOnSave = {
        command = "clippy",
      },
      cargo = {
        allFeatures = true,
      },
    },
  },
})

-- DAP (Debug Adapter Protocol) Configuration
local dap = require('dap')
dap.adapters.lldb = {
  type = 'executable',
  command = 'codelldb', -- or the path to your lldb executable
  name = 'lldb'
}

dap.configurations.rust = {
  {
    name = 'Launch',
    type = 'lldb',
    request = 'launch',
    program = function()
      return vim.fn.input('Path to executable: ', vim.fn.getcwd() .. '/target/debug/anydesk-pivot-detector', 'file')
    end,
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
    args = {},
  },
}
