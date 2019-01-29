# PasteNCServer

This is TCP server for a [paste service I developed](https://paste.nikitavbv.com).

All data from client is saved as paste and URL is sent as a response.

This allows creating pastes from terminal on Unix:

```bash
some_command | nc paste.nikitavbv.com 4242
```

or upload files:

```bash
cat some_file | nc paste.nikitavbv.com 4242
```

Pretty convenient, taking into account you can find `nc` on almost any Unix machine.
