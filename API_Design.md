# Overview
There will be two parts to this API: a GET-based data retrieval component and a
POST-based data push component

# VERY LARGE SUPER BIG IMPORTANT NOTE
THIS IS AS IT STANDS REALLY INSECURE. DON'T USE WITH CONFIDENTIAL DATA


## Config File Format
The config file will be written in JSON, and will define the format of the 
received data as well as the storing mechanic of the server

An example is shown below.
```json
{
    "server_name": "The name of the website",
    "port": 443,
    "allowed_users": [
        "Array of string UUIDs", 
        "that are allowed to connect", 
        "to the server"
    ],
    "data_format": [
        "Array of",
        "Variable names"
    ]
}
```


## GET API
The entire GET API will be based around returning JSON data. All data fetched 
through this API will be encoded in JSON. This allows for various front-ends to
be used.

The user will first log themselves in through the URL `/api/login`.
When the user has successfully authenticated, they will get a cookie with their
UUID, allowing them to access the rest of the API. 

### Getting personal stats
Path: `/api/get/<USER_UUID>/personal`

Returns JSON as defined in the config file.

### Getting other users
Path: `/api/get/<USER_UUID>/personal`

Returns a JSON array of all of the other existing users.


## POST API
Like the GET API, this will be based around POSTing JSON data. The format of the
data will be defined in the config file provided to the server, and any 
deviation from that format will be rejected. Also like the GET API, the user 
will have to successfully authenticate with the server to get the UUID of the 
user. 

### Posting a New Data Entry
Path: `/api/post/<USER_UUID>/new`

POST must be in a JSON format, and in the propper format for the data server, 
otherwise the server will reject it.
