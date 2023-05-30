# staticli

### Check the status of any website from your command line!

---

- Written in Rust

- This is my first foray into developing with Rust and creating a usable CLI tool

- Usage specifics below

---

### Usage

Use `staticli --help` for detailed information about each command.

#### Commands

    - list
    - add
    - update
    - delete
    - check

##### List

`staticli list`

This command will list out any saved aliases that have been created using the `add` command.

##### Add

`staticli add --alias <ALIAS> --endpoint <ENDPOINT>`

This command adds an alias to the configuration for easy access to check a saved site.

Example:
`staticli add --alias google --endpoint https://www.google.com`

##### Update

`staticli update --alias <ALIAS> --endpoint <ENDPOINT>`

This command updates an endpoint associated with an existing alias in the saved configuration.

Example:
`staticli update --alias google --endpoint https://www.netflix.com`

##### Delete

`staticli delete --alias <ALIAS>`

This command deletes a configured alias entry from the configuration.

Example:
`staticli delete --alias google`

##### Check

`staticli check --alias <ALIAS>`
`staticli check --endpoint <ENDPOINT>`

This command executes a status check on the given endpoint (via configured alias or by direct entry) and returns information about the request.

Example:
`staticli check --endpoint https://www.google.com`

Example response:

```
Running a check on: www.google.com

✨ Results ✨

Domain name: www.google.com
Port: 80
Status: Up 🚀
IP Address: 172.217.18.4
Status Code: 200
Response Time: 0.026
```

---

#### Features to be added

- interactive alias creation
- chained commands
  - ex: staticli add alias google
    vs staticli add --alias google
- `tail` command
  - run a check `n` times at a given interval, print out to console or file location
