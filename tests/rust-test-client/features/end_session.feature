Feature: EndSession feature

Scenario: asking server to create a new session after the previous one was ended
    Given machine manager server is up
    And machine manager server with terminated session
    When client asks machine manager server to create a new session with the same session id when forcing is disabled
    Then server returns machine hash 27688E9BDB89F7ED6C7259720D23FA93C3C2102A614A34B2FF3D42FF8B95BE6E
