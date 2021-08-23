@newsession
Feature: NewSession feature

    Scenario: NewSession asking server to create a new session
        Given cartesi machine default config description
        When client asks machine manager server to create a new session
        Then server returns machine hash 8009A430CFFD6013F9224B935DD0C9426200C59E3885DF2E46DBCD7975C61305

    Scenario: NewSession asking server to create an existing session without force
        Given cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is disabled
        Then machine manager server returns an InvalidArgument error

    Scenario: NewSession asking server to create an existing session with force
        Given cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is enabled
        Then server returns machine hash 8009A430CFFD6013F9224B935DD0C9426200C59E3885DF2E46DBCD7975C61305
