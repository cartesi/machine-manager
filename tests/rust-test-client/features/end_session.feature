Feature: EndSession feature

Scenario: asking server to create a new session after the previous one was ended
    Given machine manager server is up
    And machine manager server with terminated session
    When client asks machine manager server to create a new session with the same session id when forcing is disabled
    Then server returns machine hash E6B64F49947D3AC83F7B54E57E47081657813AB679A433B7F5D3E1D136F66072
