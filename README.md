# link

## Setup
.env needs the following
```sh
DATABASE_URL=url or uri to database
AUTH_KEY=an auth key
```
Rocket.toml
```toml
# Database
[global.databases.redirect-api]
url = "postgres://<user>:<password>@localhost/redirect-api"
```
