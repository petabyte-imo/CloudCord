# Discord Cloud Storage

This project uses discord as a form of cloud storage  
It uses [axum](https://crates.io/crates/axum) for the backend api, and [react](https://react.dev) for the front end

**Quick Note**: This wont work for files _under_ 25MB

## Prerequisites

First of all you need to download [rust](rust-lang.org/tools/install), [nodejs](https://nodejs.org/en/download) and finally you need to download [postgresql](https://www.postgresql.org/download/) and create a database using the [psql](https://www.postgresql.org/docs/current/app-psql.html#:~:text=psql%20is%20a%20terminal-based,or%20from%20command%20line%20arguments.) shell, with the following command

### Make the postgresql database

```sql
postgres=# CREATE DATABASE databasename;
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

## To use it

Make sure you've set it up alright and have the backend and frontend running  
Then Navigate to http://127.0.0.1:3000

### To upload a file

To upload a file, go to the right side of the screen, and click on the `Choose File` button  
Once you've chosen the file you want to upload, click on the `Upload File` button

- **Warning**: Once you've clicked on the `Upload File` button, don't refresh the page, as it will interrupt the uploading process and might cause the files to be left in your uploads/chunks, which won't be good for our storage  
  You'll know that the uploading process, when you'd either, see in the console for the backend `Successfully sent files`  
  Or when you'd see the file you just uploaded, in the right side of the website where, the list of the uploaded files is at

### To download a file

To download a file, go to the left side of the website, where you should see the list of uploaded files, and click on the `Download File` button, then it will process the chunks by downloading them, and reassembling them

- **Warning**: Again once you've clicked on the `Download File` button, don't refresh the page, for the same reasons stated in the [`To upload a file`](#to-upload-a-file) section

### To delete a file

And finally, to delete a file, go to the same place you went to in the [`To download a file`](#to-download-a-file) section, and this time click on the `Delete File` button, then it will delete the file from database, hence deleting it from the file list
