# shadows

Shadows (Proposals for a better name are welcome) lets you specify files that you want to shadow from a remote url.
This can be helpful if you want to use a script from a repository in your project, that should automatically update changes.

## shadows.json specification

    {
        "version": "1.0",
        "syncFiles": [
            {
                "file": "path/to/local/file",
                "remote": "https://127.0.0.1/path-to-remote-file",
                "commentPrefix": "# ",
                "suppressComments": false
            }
        ]
    }

version
: The version of the config (will be later used for backwards-compatibility)

syncFiles
: List in which every entry corresponds to a file

file
: Local path where the file will be downloaded to (relative to the parent folder of the config)

remote
: URL where the file will be downloaded from

commentPrefix
: Prefix used for info text at the beginning of the file (Default: '# ')

suppressComments
: If set to true, there will be no info text at the beginning of the file (Default: false)
