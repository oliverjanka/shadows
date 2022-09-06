# remote-sync

## remote-sync.json specification

    {
        "version": 1.0,
        "syncFiles": [
            {
                "file": "path/to/local/file",
                "remote": "https://127.0.0.1/path-to-remove-file",
                "commentPrefix": "# ",
                "suppressComments": false
            }
        ]
    }