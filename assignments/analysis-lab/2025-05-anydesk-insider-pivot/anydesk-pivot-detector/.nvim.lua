-- Neovim Local Configuration for AnyDesk Pivot Detector
-- Optimized for Rust (Backend) and React/TS (Frontend)

local function setup_lsp()
  local status_ok, lspconfig = pcall(require, 'lspconfig')
  if not status_ok then return end

  -- 1. Rust Analyzer Setup
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

  -- 2. Frontend (TypeScript/React) Setup
  if pcall(require, 'vtsls') then
    lspconfig.vtsls.setup({})
  elseif pcall(require, 'tsserver') then
    lspconfig.tsserver.setup({})
  end

  -- 3. Tailwind CSS & ESLint
  lspconfig.tailwindcss.setup({})
  lspconfig.eslint.setup({})
end

local function setup_dap()
  local status_ok, dap = pcall(require, 'dap')
  if not status_ok then return end

  dap.adapters.lldb = {
    type = 'executable',
    command = 'codelldb', 
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
end

-- 4. Auto-formatting (simple implementation if conform.nvim is not present)
vim.api.nvim_create_autocmd("BufWritePre", {
  pattern = { "*.rs", "*.ts", "*.tsx", "*.json" },
  callback = function()
    vim.lsp.buf.format({ async = false })
  end,
})

setup_lsp()
setup_dap()
