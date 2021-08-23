Feature: EndSession feature

    Scenario: EndSession asking server to create a new session after the previous one was ended
        Given machine manager server with terminated session
        When client asks machine manager server to create a new session with the same session id when forcing is disabled
        Then server returns machine hash 8009A430CFFD6013F9224B935DD0C9426200C59E3885DF2E46DBCD7975C61305
