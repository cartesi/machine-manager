Feature: SessionStoreLoad feature

    Scenario: SessionStoreLoad perform regular store-load
        Given a pristine machine manager server session
        When asking machine manager server to store the machine in a directory /stored_machine
        Then machine manager server is able to load machine from this directory correctly
        And machine manager server is able to execute this machine for 60 cycles
        And server returns machine hashes:
            | cycles | hash                                                             |
            |   60   | 1DEC67CBB473B140979EB892A86412CD8FEBC5F6394EDE2BC2D0728EC0FBC73B |
