Feature: NewSession feature

    Scenario: asking server to create a new session
        Given machine manager server is up
        And cartesi machine default config description
        When client asks machine manager server to create a new session
        Then server returns correct machine hash

    Scenario: asking server to create an existing session without force
        Given machine manager server is up
        And cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is disabled
        Then machine manager server returns an InvalidArgument error

    Scenario: asking server to create an existing session with force
        Given machine manager server is up
        And cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is enabled
        Then server returns correct machine hash
