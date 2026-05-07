# Support Portal

End user APIs and frontend for self-service data access.

## Support CLI
### Getting Started
1. Download the latest release of `support-cli` or run `cargo run -p support-cli`
2. Configure the API host with `support-cli config set host https://support-api.shared.oxide.computer`
3. Choose an authentication mode based on the kind of session you want, either a short-term session token (id) or a long-term api token (token).

#### Authenticate with short lived session
To log in with a short lived session run:
```sh
support-cli auth login oauth zendesk
```

### Authenticate and retrieve an IdP token
To retrieve an IdP token run:
```sh
support-cli auth login oauth --request-idp-token zendesk
```

In addition to logging in, this command will also retrieve an IdP token and present it back. This can be used to interacte directly with the IdP's API.
