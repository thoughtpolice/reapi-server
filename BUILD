
TOOLS = {
    'buck2-cache-server': '//tools/cache-server:cache-server',
    'buck2-smoltar':      '//tools/smoltar:smoltar',
    'buck2-upload-logs':  '//tools/logs:upload-logs',
    'buck2-log-server':   '//tools/logs:log-server',
}

filegroup(
    name = 'bucktools',
    srcs = {
        f'{name}.exe': tgt for name, tgt in TOOLS.items()
    },
)
