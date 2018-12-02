## DEPLOYMENT

-   Generate a random key

```bash
openssl rand -base64 32
```

-   Current timestamp

```bash
date +%Y%m%d%H%M%S%N
```

-   Create an Authentication Key-pair

```bash
ssh-keygen -b 4096
```

-   Upload the public key

```bash
ssh-copy-id user@host
```

-   SSH Daemon Options

```text
PermitRootLogin no # Disallow root logins over SSH
PasswordAuthentication no # Disable SSH password authentication
```

-   Determine Running Services

```bash
sudo ss -atpu
```
