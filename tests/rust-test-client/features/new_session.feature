Feature: NewSession feature

    Scenario: asking server to create a new session
        Given machine manager server is up
        And cartesi machine default config description
        When client asks machine manager server to create a new session
        Then server returns machine hash E6B64F49947D3AC83F7B54E57E47081657813AB679A433B7F5D3E1D136F66072

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
        Then server returns machine hash E6B64F49947D3AC83F7B54E57E47081657813AB679A433B7F5D3E1D136F66072
