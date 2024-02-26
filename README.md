# Crab Watch

A small full-stack pet project to tinker with Vue, Actix-web and Surrealdb

The Actix-web backend is mostly based on the book
_[Zero To Production In Rust]_ by [Luca Palmieri] that I strongly recommend.

You need a `.env` file that you can modify according to your needs:

```bash
APP_ENVIRONMENT=production
APP_DATABASE_USERNAME=root
APP_DATABASE_PASSWORD=root
APP_DATABASE_NAMESPACE=test
APP_DATABASE_DBNAME=test
VUE_APP_API_BASE_URL=http://localhost
VUE_APP_API_BASE_PORT=3000
```

<!--references-->
[Zero To Production In Rust]: https://www.zero2prod.com/index.html
[Luca Palmieri]: https://github.com/LukeMathWalker
