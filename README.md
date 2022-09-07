# Motley - Patterns for mocking architecture
Let's do an actor mocking library that
1. Validates the soundness of the mocked actors by the fact that it actually compiles
2. Validates the patterns of actor interactions with a simple test suite.

## Goals
* Mocking
  * Cloud Deployment types (Service, Function, etc)
  * Cloud Deployment patterns (HTTP, PubSub, etc)
  * Database types (SQL, NoSQL, etc)
  * Database patterns (CRUD, etc)
  * Cloud Providers types (AWS, GCP, Azure, etc)
* Generation
  * Generate common specification formats (OpenAPI, AsyncAPI, etc)
  * Generate common deployment formats (Terraform, CloudFormation, etc)
  * Generate a mock server that can be used to test the patterns between the mocked actors
* Testing
  * cargo build successful? -> Mocks are sound
  * cargo test successful? -> patterns are sound
  * Test request flows between the mocked actors
  * Ergonomic and focused on validating the patterns between the mocked actors
* Visualization
  * Visualize the mocked actors
  * Visualize the depencies between the mocked actors
  * Visualize the patterns between the mocked actors

## Stretch Goals
* Generation
  * Generate mocks from a real system
  * Generate a mock from a set of patterns
* Save/Load
  * the "save file" will be a compilable rust bin that can be used to test the patterns
    * the root of the crate will handle wiring up the mocks for the patterns
    * each actor will be represented as a separate sibling sub module
    * each actor module will have sub modules encapsulating their dependant actors
  * Save the mocked actors to a file
  * Load the mocked actors from a file

## Non-Goals
```rs
todo!()
```

# TURN BACK NOW

## Ranting by the author and some ideas and motivations

Visualize in the same manner as `mdBook`

impl as traits with enum impls?
generate from OpenAPI spec?
output to OpenAPI spec?
output to OpenAPI spec with examples?
output to ORM tooling?
output to Rust framework?
output to Go framework?
outputting a sub implementation to a lang or spec is a good idea and can help spin up complicated systems

RPC Service
* HTTP
  * Request Resource (common endpoint)
    * Nesting?
    * List - GET /resource
    * Show - GET /resource/:id
    * Create - POST /resource
    * Update - PUT /resource/:id
    * Delete - DELETE /resource/:id
  * Request Object
    * Headers (struct)
    * Path Parameters (struct)
    * Query Parameters (struct)
    * Body (struct)
  * Response Object
    * Headers (struct)
    * Status Code (enum)
    * Body (struct)
* gRPC
  * Service Object
    * RPC Methods
      * Request Object
        * Metadata (struct)
        * Message (struct)
      * Response Object
        * Metadata (struct)
        * Message (struct)
    * Stream Methods
      * Request Object
        * Metadata (struct)
        * Message (struct)
      * Response Object
        * Metadata (struct)
        * Message (struct)

Database Service
* Relational
  * Repository pattern
    * Specific highlevel functions defining the CRUD operations
  * ORM pattern (LATER)
    * Generic CRUD operations only
    * Likely a late stage goal due to the complexity of the ORM pattern
  * Raw SQL (LATER)
    * SELECT, INSERT, UPDATE, DELETE
* NoSQL
  * Key/Value
  * Document
  * Graph
  * Columnar
  * Time Series
  * Search
* Caching layer?

Message Queue Service
* Pull based
  * Pull - GET /queue/:queue_name
  * Ack - POST /queue/:queue_name
  * Nack - POST /queue/:queue_name
* Push based
  * Push -> Ack/Nack - POST /queue/:queue_name
* Request Object
  * Metadata
  * Body
