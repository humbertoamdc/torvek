## Todo

### Api

- [ ] Handle sessions.
    - [x] Create an endpoint to retrieve session information
    - [ ] Create a middleware to retrieve sessions from identity manager service.
- [ ] Add app state and pass that to the controllers.
- [ ] Error handling. Use thiserror crate.
    - [x] Map all ory errors to custom api errors. Our custom error should have an error message and an status code.
    - [x] Replace all `expect()` with `?` for error propagation.
    - [ ] Handle serialization errors in ory implementation.
    - [ ] (Maybe) Add custom struct to store the errors with message, code and status code.
    - [ ] Transform errors in controller to a presentable http version.
    - [ ] Set correct error status code in controllers.
- [ ] Add documentation for:
    - [ ] Imported structs
    - [ ] Imported functions
    - [ ] Controllers
- [ ] Constraint imports. Each mod file should only include members required by other packages.
- [ ] Add tests.

### Infra

- [x] Create AWS account.
- [ ] Dockerize app.
- [ ] Deploy docker image to a lambda and create required infra (LB/API Gateway)