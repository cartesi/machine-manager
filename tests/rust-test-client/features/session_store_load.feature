Feature: SessionStoreLoad feature

    Scenario: perform regular store-load
        Given machine manager server is up
        And a pristine machine manager server session
        When asking machine manager server to store the machine in a directory /stored_machine
        Then machine manager server is able to load machine from this directory correctly
        And machine manager server is able to execute this machine for 60 cycles
        And server returns machine hashes:
            | cycles | hash                                                             |
            |   60   | 2A263E0E58691C40E6F345BBCFD5BA307F7DD0F4CED6D84CE1DEEB3765A2102B |
