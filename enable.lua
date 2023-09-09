function restart_htmx_lsp()
    require("lsp-debug-tools").restart({ expected = {}, name = "scss_lsp", cmd = { "htmx-lsp", "--level", "DEBUG" }, root_dir = vim.loop.cwd(), });
end
