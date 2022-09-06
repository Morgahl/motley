/*  Let's do a system mocking library that validates the soundness of the interactions by the fact
    that it actually compiles?!?!?! AND IN RUST!!!!!!

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
*/
