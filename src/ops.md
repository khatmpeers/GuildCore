# Operations

## Client
- `Draft`: Draft a request without actually uploading it.
    - Depends on: Core
- `Publish`: Publish a drafted request
    - Depends on: *Reward*
- `Create`: Directly create a request
    - Depends on: Core + *Reward*
- `Rescind`: Recall a previously uploaded request. Only works if `!request.elevated`.

