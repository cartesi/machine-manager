Feature: SessionStoreLoad feature

    Scenario: perform regular store-load
        Given machine manager server is up
        And a pristine machine manager server session
        When asking machine manager server to store the machine in a directory /stored_machine
        Then machine manager server is able to load machine from this directory correctly
        And machine manager server is able to execute this machine for 60 cycles
        And server returns correct machine hashes
