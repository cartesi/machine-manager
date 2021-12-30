Feature: EndSession feature

Scenario: asking server to create a new session after the previous one was ended
    Given machine manager server is up
    And machine manager server with terminated session
    When client asks machine manager server to create a new session with the same session id when forcing is disabled
    Then server returns correct machine hash
