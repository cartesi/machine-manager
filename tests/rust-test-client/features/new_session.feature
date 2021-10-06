Feature: NewSession feature

    Scenario: asking server to create a new session
        Given machine manager server is up
        And cartesi machine default config description
        When client asks machine manager server to create a new session
        Then server returns machine hash 27688E9BDB89F7ED6C7259720D23FA93C3C2102A614A34B2FF3D42FF8B95BE6E

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
        Then server returns machine hash 27688E9BDB89F7ED6C7259720D23FA93C3C2102A614A34B2FF3D42FF8B95BE6E
