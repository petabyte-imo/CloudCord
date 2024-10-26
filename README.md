# CloudCord

This project uses discord as a form of cloud storage  
It uses [axum](https://crates.io/crates/axum) for the backend api, and [react](https://react.dev) for the front end

## WARNING
**This project counts as API Abuse, which is against discord's developer TOS, use at your own risk!!**

## Prerequisites

First of all you need to download [rust](rust-lang.org/tools/install), [nodejs](https://nodejs.org/en/download) and finally you need to download [postgresql](https://www.postgresql.org/download/) and create a database using the [psql](https://www.postgresql.org/docs/current/app-psql.html#:~:text=psql%20is%20a%20terminal-based,or%20from%20command%20line%20arguments.) shell, with the following command

### Make the postgresql database

While in the [psql](https://www.postgresql.org/docs/current/app-psql.html#:~:text=psql%20is%20a%20terminal-based,or%20from%20command%20line%20arguments.) shell run the following command

```sql
CREATE DATABASE databasename;
```

**Note**: Make sure to change the `databasename` to the name of the database you want it to be

### Make the needed directories

And you also need to make a uploads and uploads/chunks directory with this command (works in linux, macos and windows)  
First navigate to the directory of the repo

```sh
$ cd CloudCord
```

Then make the needed directories

```sh
mkdir uploads
```

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

```sh
$ cd CloudCord
```

Then you have to run the release build with this command

```sh
$ cargo run -- --release
```

### Run the frontend

Open a new terminal  
Then build and run it with three simple commands  
First of all you need to change directory into the directory of the repo then the frontend directory

```sh
$ cd CloudCord/frontend
```

Then you have to install the dependancies and run the release build with this command

```sh
$ npm install
```

```sh
$ npm run build
```

After you've ran the release build, follow the instructions it gives  
**Note:** if it gives an error, try installing the react-scripts dependency globally with this command

```sh
$ npm i -g react-scripts
```

## To use it

Make sure you've set it up alright and have the backend and frontend running  
Then Navigate to http://127.0.0.1:3000

### To upload a file

To upload a file, first of all select if you want it the contents to be encrypted or not, by clicking on the checkbox  
And also set the encryption key, can be any string smaller than 32 characters  
Then go to the top right of the table, and click on the `Choose File` button  
Once you've chosen the file you want to upload, click on the `Upload File` button

- **Warning**: Once you've clicked on the `Upload File` button, don't refresh the page, as it will interrupt the uploading process and might cause the files to be left in your uploads/chunks, which won't be good for our storage  
  You'll know that the uploading process, when you'd either, see in the console for the backend `Successfully sent files`  
  Or when you'd see the file you just uploaded, in the right side of the website where, the list of the uploaded files is at

### To download a file

Go to the table, where you should see the list of uploaded files, and click on the `Download File` button, which is in the `Actions` column, it will then start processing the chunks by downloading them, and reassembling them  
If the file is encrypted it should give you a popup where you enter the key to decrypt that file, and then click on the `Download File` button, and proceed how you normally would

- **Warning**: Again once you've clicked on the `Download File` button, don't refresh the page, for the same reasons stated in the [`To upload a file`](#to-upload-a-file) section

### To delete a file

And finally, to delete a file, go to the same place you went to in the [`To download a file`](#to-download-a-file) section, and this time click on the `Delete File` button, then it will delete the file from database, hence deleting it from the file list
