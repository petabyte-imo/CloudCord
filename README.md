# Discord Cloud Storage

This project uses discord as a form of cloud storage  
It uses [axum](https://crates.io/crates/axum) for the backend api, and [react](https://react.dev) for the front end

## Prerequisites

First of all you need to download [rust](rust-lang.org/tools/install), [nodejs](https://nodejs.org/en/download) and finally you need to download [postgresql](https://www.postgresql.org/download/) and create a database using the [psql](https://www.postgresql.org/docs/current/app-psql.html#:~:text=psql%20is%20a%20terminal-based,or%20from%20command%20line%20arguments.) shell, with the following command

### Make the postgresql database

```sql
postgres=# CREATE DATABASE new_database;
```

### Make the needed directories

And you also need to make a uploads and uploads/chunks directory with this command (works in linux, macos and windows)  
First navigate to the directory of the repo

```sh
$ cd discord-cloud-storage
```

Then make the directory

```sh
mkdir uploads/chunks
```

### Make a Secrets.toml

To keep stuff private, and easy to use, I decided to use a Secrets.toml  
It fetches the data needed from there  
So go ahead and make a **Secrets.toml** file in the root directory of the project
It should have these fields

```toml
DATABASE_URL="postgresql://username:password@localhost:5432/databasename"
# The url is formatted like above
BOT_TOKEN="Your discord bot token"
CHANNEL_ID="Channel ID to upload the files to as a form of storage"
```

## To run on linux, macos and windows

### Run the backend

You can build and run it with two simple commands  
First of all you need to change directory into the directory of the repository

Then you have to run the release build with this command

```sh
$ cargo run -- --release
```

### Run the frontend

You can build and run it with two simple commands  
First of all you need to change directory into the directory of the repo then the frontend directory

```sh
$ cd discord-cloud-storage/frontend
```

Then you have to run the release build with this command

```sh
$ npm run build
```

**Note:** if it gives an error, try installing the react-scripts dependency globally with this command

```sh
$ npm i -g react-scripts
```

### To use it

Navigate to http://127.0.0.1:3000  
And voila you can now use it to upload and download the files you've uploaded
